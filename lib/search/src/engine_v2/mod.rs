pub mod names;
pub mod result_item;
pub mod words;

use std::{cmp::min, collections::BinaryHeap, marker::PhantomData, thread};

use config::Config;
use error::Error;
use resources::{models::storage::ResourceStorage, parse::jmdict::languages::Language};
use vector_space_model::{
    document_vector, metadata::Metadata, traits::Decodable, DocumentVector, Index,
};

use self::result_item::ResultItem;

/// Load all indexes for word search engine
pub fn load_indexes(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut joins = Vec::with_capacity(5);

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        words::native::index::load(config1.get_indexes_source());
    }));

    let config1 = config.clone();
    joins.push(thread::spawn(move || {
        names::native::index::load(&config1);
    }));

    for j in joins {
        j.join().map_err(|_| error::Error::Unexpected)?;
    }

    /*
    word::japanese::index::load(config.get_indexes_source());
    name::foreign::index::load(config);
    sentences::japanese::index::load(config);
    sentences::foreign::index::load(config)?;
    */

    Ok(())
}

pub trait SearchEngine: Indexable {
    type GenDoc: document_vector::Document;
    type Output: PartialEq;

    /// Loads the corresponding Output type from a document
    fn doc_to_output<'a>(
        storage: &'a ResourceStorage,
        input: &Self::Document,
    ) -> Option<Vec<&'a Self::Output>>;

    /// Generates a vector for a query, in order to be able to compare results with a vector
    fn gen_query_vector(
        index: &Index<Self::Document, Self::Metadata>,
        query: &str,
    ) -> Option<DocumentVector<Self::GenDoc>>;

    #[inline]
    fn align_query(
        &self,
        _original: &str,
        _index: &Index<Self::Document, Self::Metadata>,
    ) -> Option<&str> {
        None
    }
}

pub trait Indexable {
    type Metadata: Metadata + 'static;
    type Document: Decodable + Clone + 'static + PartialEq;

    fn get_index(
        language: Option<Language>,
    ) -> Option<&'static Index<Self::Document, Self::Metadata>>;
}

pub struct SerachTask<'a, T>
where
    T: SearchEngine,
{
    query: &'a str,
    language: Option<Language>, // To pick the correct index to search in
    vec_filter: Option<Box<dyn Fn(&T::Document) -> bool>>,
    res_filter: Option<Box<dyn Fn(&T::Output) -> bool>>,
    threshold: f32,
    limit: usize,
    total_limit: usize,
    offset: usize,
    phantom: PhantomData<T>,
}

impl<'a, T> SerachTask<'a, T>
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

    /// Sets the search task's threshold
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

    /// Set the serach task's vector filter.
    pub fn set_vector_filter<F: 'static>(&mut self, vec_filter: F)
    where
        F: Fn(&T::Document) -> bool,
    {
        self.vec_filter = Some(Box::new(vec_filter));
    }

    /// Set the serach task's result filter.
    pub fn set_result_filter<F: 'static>(&mut self, res_filter: F)
    where
        F: Fn(&T::Output) -> bool,
    {
        self.res_filter = Some(Box::new(res_filter));
    }

    /// Returns `true` if the search task's query is a term in the corresponding index
    #[inline]
    pub fn has_term(&self) -> bool {
        T::get_index(self.language)
            .map(|i| i.get_indexer().clone().find_term(self.query).is_some())
            .unwrap_or(false)
    }

    /// Runs the search task and returns the result.
    pub fn find(&self) -> Result<(Vec<ResultItem<&T::Output>>, usize), Error> {
        let index = match T::get_index(self.language) {
            Some(index) => index,
            None => return Ok((vec![], 0)),
        };

        let vec = T::gen_query_vector(index, self.query).ok_or(error::Error::NotFound)?;
        self.find_by_vec(vec)
    }

    fn find_by_vec(
        &self,
        vec: DocumentVector<T::GenDoc>,
    ) -> Result<(Vec<ResultItem<&T::Output>>, usize), Error> {
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
                self.language
                    .map(|i| ResultItem::with_language(item, rel, i))
                    .unwrap_or(ResultItem::new(item, rel))
            })
            .take(self.total_limit)
            .collect::<Vec<_>>();

        let mut heap: BinaryHeap<ResultItem<&T::Output>> = BinaryHeap::from(items);

        let len = heap.len();
        let out = self.get_items_from_heap(&mut heap);
        Ok((out, len))
    }

    /// Returns the correct items from the heap. This includes `offset` and `limit`. The length of
    /// the returned vector is always equal or smaler than `limit`
    fn get_items_from_heap<O: Ord>(&self, heap: &mut BinaryHeap<O>) -> Vec<O> {
        if self.offset >= heap.len() {
            return vec![];
        }
        let item_count = min(heap.len() - self.offset, self.limit);
        let mut out = Vec::with_capacity(item_count);

        for _ in 0..self.offset {
            heap.pop();
        }

        for _ in 0..item_count {
            out.push(heap.pop().unwrap());
        }

        out
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

impl<'a, T: SearchEngine> Default for SerachTask<'a, T> {
    #[inline]
    fn default() -> Self {
        Self {
            query: "",
            language: None,
            vec_filter: None,
            res_filter: None,
            threshold: 0.2,
            limit: 1000,
            total_limit: 2000,
            offset: 0,
            phantom: PhantomData::default(),
        }
    }
}
