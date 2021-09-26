use super::{result::SearchResult, result_item::ResultItem, SearchEngine};
use error::Error;
use resources::parse::jmdict::languages::Language;
use std::{collections::BinaryHeap, marker::PhantomData};
use vector_space_model::DocumentVector;

pub struct SearchTask<'a, T>
where
    T: SearchEngine,
{
    /// Search query
    query: &'a str,
    language: Option<Language>, // To pick the correct index to search in
    /// filter out vectors
    vec_filter: Option<Box<dyn Fn(&T::Document) -> bool>>,
    /// Filter out results
    res_filter: Option<Box<dyn Fn(&T::Output) -> bool>>,
    /// Custom result order function
    order: Option<Box<dyn Fn(&T::Output, f32) -> usize>>,
    /// Min relevance returned from vector space algo
    threshold: f32,
    limit: usize,
    total_limit: usize,
    offset: usize,
    phantom: PhantomData<T>,
}

impl<'a, T> SearchTask<'a, T>
where
    T: SearchEngine,
{
    /// Creates a new Search engine
    #[inline]
    pub fn new(query: &'a str) -> Self {
        let mut task = Self::default();
        task.query = query;
        task
    }

    /// Set the language to search in. This'll only have an effect if the used SearchEngine
    /// supports languages
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Set the total limit. This is the max amount of vectors which will be loaded and processed
    pub fn limit(mut self, total_limit: usize) -> Self {
        self.limit = total_limit;
        self
    }

    /// Sets the search task's threshold. This does not apply on the final score, which can be
    /// overwritten by `order` but applies to the vector space relevance itself.
    pub fn threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    /// Sets the offeset of the search. Can be used for pagination. Requires output of search being
    /// directly used and not manually reordered
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// Set the search task's vector filter.
    pub fn set_vector_filter<F: 'static>(&mut self, vec_filter: F)
    where
        F: Fn(&T::Document) -> bool,
    {
        self.vec_filter = Some(Box::new(vec_filter));
    }

    /// Set the search task's result filter.
    pub fn set_result_filter<F: 'static>(&mut self, res_filter: F)
    where
        F: Fn(&T::Output) -> bool,
    {
        self.res_filter = Some(Box::new(res_filter));
    }

    /// Set the search task's custom order function
    pub fn set_order_fn<F: 'static>(&mut self, res_filter: F)
    where
        F: Fn(&T::Output, f32) -> usize,
    {
        self.order = Some(Box::new(res_filter));
    }

    /// Returns `true` if the search task's query is a term in the corresponding index
    #[inline]
    pub fn has_term(&self) -> bool {
        T::get_index(self.language)
            .map(|i| i.get_indexer().clone().find_term(self.query).is_some())
            .unwrap_or(false)
    }

    /// Runs the search task and returns the result.
    pub fn find(&self) -> Result<SearchResult<&T::Output>, Error> {
        let index = match T::get_index(self.language) {
            Some(index) => index,
            None => return Ok(SearchResult::default()),
        };

        let vec = T::gen_query_vector(index, self.query).ok_or(error::Error::NotFound)?;
        self.find_by_vec(vec)
    }

    fn find_by_vec(
        &self,
        vec: DocumentVector<T::GenDoc>,
    ) -> Result<SearchResult<&T::Output>, Error> {
        let index = T::get_index(self.language);
        if index.is_none() {
            log::error!("Failed to retrieve {:?} index with language", self.language);
            return Err(Error::Unexpected);
        }
        let index = index.unwrap();

        // Retrieve all document vectors that share at least one dimension with the query vector
        let document_vectors = index
            .get_vector_store()
            .clone()
            .get_all(&vec.vector().vec_indices().collect::<Vec<_>>())
            .ok_or(error::Error::NotFound)?;

        let storage = resources::get();

        let items = document_vectors
            .into_iter()
            .filter_map(|i| {
                if !self.filter_vector(&i.document) {
                    return None;
                }

                let similarity = i.similarity(&vec);
                if similarity <= self.threshold {
                    return None;
                }

                // Retrieve `Output` values for given documents
                let res = T::doc_to_output(storage, &i.document)?
                    .into_iter()
                    .map(move |i| (similarity, i));

                Some(res)
            })
            .flatten()
            .filter(|i| self.filter_result(&i.1))
            .map(|(rel, item)| {
                let relevance = self.calculate_score(item, rel);

                self.language
                    .map(|i| ResultItem::with_language(item, relevance, i))
                    .unwrap_or(ResultItem::new(item, relevance))
            })
            .take(self.total_limit)
            .collect::<Vec<_>>();

        let heap: BinaryHeap<ResultItem<&T::Output>> = BinaryHeap::from(items);

        Ok(SearchResult::from_binary_heap(
            heap,
            self.offset,
            self.limit,
        ))
    }

    /// Calculates the score using a custom function if provided or just `rel` otherwise
    #[inline]
    fn calculate_score(&self, item: &T::Output, rel: f32) -> usize {
        self.order
            .as_ref()
            .map(|i| i(item, rel))
            .unwrap_or((rel * 100f32) as usize)
    }

    #[inline]
    fn filter_result(&self, output: &T::Output) -> bool {
        self.res_filter.as_ref().map(|i| i(output)).unwrap_or(true)
    }

    #[inline]
    fn filter_vector(&self, vec: &T::Document) -> bool {
        self.vec_filter.as_ref().map(|i| i(vec)).unwrap_or(true)
    }
}

impl<'a, T: SearchEngine> Default for SearchTask<'a, T> {
    #[inline]
    fn default() -> Self {
        Self {
            query: "",
            language: None,
            vec_filter: None,
            res_filter: None,
            order: None,
            threshold: 0.2,
            limit: 1000,
            total_limit: 2000,
            offset: 0,
            phantom: PhantomData::default(),
        }
    }
}
