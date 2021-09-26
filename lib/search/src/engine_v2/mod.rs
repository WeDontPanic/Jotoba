pub mod result_item;
pub mod words;

use std::{cmp::Ordering, marker::PhantomData, thread};

use config::Config;
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

    for j in joins {
        j.join().map_err(|_| error::Error::Unexpected)?;
    }

    /*
    word::japanese::index::load(config.get_indexes_source());
    name::japanese::index::load(config);
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
    ) -> Option<&'a Self::Output>;

    /// Generates a vector for a query, in order to be able to compare results with a vector
    fn gen_query_vector(
        &self,
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
    total_limit: u32,
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
    pub fn with_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    /// Set the total limit. This is the max amount of vectors which will be loaded and processed
    pub fn with_total_limit(&mut self, total_limit: u32) -> &mut Self {
        self.total_limit = total_limit;
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

    /// Sets the search task's threshold
    pub fn with_threshold(&mut self, threshold: f32) -> &mut Self {
        self.threshold = threshold;
        self
    }

    pub fn find_by_vec(
        &self,
        vec: DocumentVector<T::GenDoc>,
    ) -> Result<Vec<ResultItem<&T::Output>>, Box<dyn std::error::Error>> {
        let index = match T::get_index(self.language) {
            Some(index) => index,
            None => return Ok(vec![]),
        };

        // Retrieve all document vectors that share at least one dimension with the query vector
        let document_vectors = index
            .get_vector_store()
            .clone()
            .get_all(&vec.vector().vec_indices().collect::<Vec<_>>())
            .ok_or(error::Error::NotFound)?;

        // Sort by relevance
        let mut found: Vec<_> = document_vectors
            .iter()
            .filter(|i| self.filter_vector(&i.document))
            .filter_map(|i| {
                let similarity = i.similarity(&vec);
                (similarity >= self.threshold).then(|| (&i.document, similarity))
            })
            .collect();

        // Sort by similarity to top
        //found.sort_by(|a, b| sort_fn(a, b));
        found.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal).reverse());
        //found.dedup_by(|a, b| a.document == b.document);

        let storage = resources::get();

        // Convert DocumentVectors to ResultItems
        let res = found
            .into_iter()
            .take(self.total_limit as usize)
            .filter_map(|(doc, rel)| {
                let item = T::doc_to_output(storage, doc)?;

                // Filter results
                if !self.filter_result(item) {
                    return None;
                }

                let res = self
                    .language
                    .map(|i| ResultItem::with_language(item, rel, i))
                    .unwrap_or(ResultItem::new(item, rel));

                Some(res)
            })
            .collect::<Vec<_>>();

        Ok(res)
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
            total_limit: 2000,
            phantom: PhantomData::default(),
        }
    }
}
