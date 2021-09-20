pub mod index;

use self::index::Index;

use crate::engine::{
    document::SingleDocument,
    result::{ResultItem, SearchResult},
    simple_gen_doc::GenDoc,
    FindExt,
};
use error::Error;
use resources::parse::jmdict::languages::Language;
use vector_space_model::DocumentVector;

pub struct Find<'a> {
    limit: usize,
    offset: usize,
    query: &'a str,
    treshold: f32,
}

impl<'a> Find<'a> {
    #[inline]
    pub fn new(query: &'a str, limit: usize, offset: usize) -> Self {
        Self {
            limit,
            offset,
            query,
            treshold: 0f32,
        }
    }

    #[inline]
    pub fn with_treshold(mut self, treshold: f32) -> Self {
        self.treshold = treshold;
        self
    }

    /// Do a foreign word search
    pub async fn find(&self) -> Result<SearchResult, Error> {
        let index = index::INDEX.get().ok_or(Error::Unexpected)?;

        let query_vec = match self.gen_query(&index) {
            Some(query) => query,
            None => return Ok(SearchResult::default()),
        };

        self.find_by_vec(query_vec).await
    }

    /// Do a foreign word search with a custom `query_vec`
    pub async fn find_by_vec(
        &self,
        query_vec: DocumentVector<GenDoc>,
    ) -> Result<SearchResult, Error> {
        let index = index::INDEX.get().ok_or(Error::Unexpected)?;

        // VecStore is surrounded by an Arc
        let mut doc_store = index.get_vector_store().clone();

        // All vectors in queries dimensions
        let dimensions = query_vec.vector().vec_indices().collect::<Vec<_>>();

        // Retrieve all matching vectors
        let document_vectors = doc_store
            .get_all_async(&dimensions)
            .await
            .map_err(|_| error::Error::NotFound)?;

        let mut items = self
            .vecs_to_result_items(&query_vec, &document_vectors, self.treshold)
            .into_iter()
            .map(|doc| ResultItem {
                seq_id: doc.document.seq_id,
                relevance: doc.relevance,
                language: Language::English,
            })
            .collect::<Vec<_>>();

        let has_relevant = items.iter().any(|i| i.relevance > 0.1);
        if has_relevant {
            items.retain(|i| i.relevance > 0.1);
        }

        Ok(SearchResult::new(items))
    }

    /// Generate a document vector out of `query_str`
    fn gen_query(&self, index: &Index) -> Option<DocumentVector<GenDoc>> {
        let query_document = GenDoc::new(vec![self.query.to_string()]);
        let mut doc = DocumentVector::new(index.get_indexer(), query_document.clone())?;

        // TODO: look if this makes the results really better. If not, remove
        let terms = tinysegmenter::tokenize(self.query);
        doc.add_terms(index.get_indexer(), &terms, true, Some(0.03));

        Some(doc)
    }
}

impl<'a> FindExt for Find<'a> {
    type ResultItem = ResultItem;
    type GenDoc = GenDoc;
    type Document = SingleDocument;

    #[inline]
    fn get_limit(&self) -> usize {
        self.limit
    }

    #[inline]
    fn get_offset(&self) -> usize {
        self.offset
    }

    #[inline]
    fn get_query_str(&self) -> &str {
        &self.query
    }
}