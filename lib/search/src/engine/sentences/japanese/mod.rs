pub(crate) mod document;
pub(crate) mod index;

use error::Error;
use resources::parse::jmdict::languages::Language;
use vector_space_model::DocumentVector;

use crate::engine::{
    result::{ResultItem, SearchResult},
    simple_gen_doc::GenDoc,
    FindExt,
};

use self::{document::SentenceDocument, index::Index};

pub(crate) struct Find<'a> {
    limit: usize,
    offset: usize,
    query: &'a str,
    language_filter: Option<Language>,
    show_english: bool,
}

impl<'a> FindExt for Find<'a> {
    type ResultItem = ResultItem;
    type GenDoc = GenDoc;
    type Document = SentenceDocument;

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

impl<'a> Find<'a> {
    #[inline]
    pub(crate) fn new(query: &'a str, limit: usize, offset: usize) -> Self {
        Self {
            limit,
            offset,
            query,
            language_filter: None,
            show_english: false,
        }
    }

    /// Only find sentences which have a certain language
    #[inline]
    pub(crate) fn with_language_filter(mut self, lanuage: Language) -> Self {
        self.language_filter = Some(lanuage);
        self
    }

    /// Also show english translations, next to potentially filtered languages
    #[inline]
    pub(crate) fn find_engish(mut self, show_english: bool) -> Self {
        self.show_english = show_english;
        self
    }

    /// Do a foreign word search
    pub(crate) async fn find(&self) -> Result<SearchResult, Error> {
        let index = index::INDEX.get().ok_or(Error::Unexpected)?;

        let query_vec = match self.gen_query(&index) {
            Some(query) => query,
            None => return Ok(SearchResult::default()),
        };

        self.find_by_vec(query_vec).await
    }

    /// Do a foreign word search with a custom `query_vec`
    pub(crate) async fn find_by_vec(
        &self,
        query_vec: DocumentVector<GenDoc>,
    ) -> Result<SearchResult, Error> {
        let index = index::INDEX.get().ok_or(Error::Unexpected)?;

        // VecStore is surrounded by an Arc
        let mut doc_store = index.get_vector_store().clone();

        // All vectors in queries dimensions
        let dimensions = query_vec.vector().vec_indices().collect::<Vec<_>>();

        // Retrieve all matching vectors
        let mut document_vectors = doc_store
            .get_all_async(&dimensions)
            .await
            .map_err(|_| error::Error::NotFound)?;

        if let Some(language_filter) = self.language_filter {
            document_vectors.retain(|item| {
                item.document.has_language(language_filter)
                    || (self.show_english && item.document.has_language(Language::English))
            });
        }

        let result = self
            .vecs_to_result_items(&query_vec, &document_vectors, 0f32)
            .into_iter()
            .map(|i| {
                let seq_id = i.document.seq_id as usize;
                let relevance = i.relevance;
                ResultItem {
                    seq_id,
                    relevance,
                    language: Language::English,
                }
            })
            .collect();

        Ok(SearchResult::new(result))
    }

    /// Generate a document vector out of `query_str`
    #[inline]
    fn gen_query(&self, index: &Index) -> Option<DocumentVector<GenDoc>> {
        let terms = tinysegmenter::tokenize(self.query);
        let query_document = GenDoc::new(terms);
        DocumentVector::new(index.get_indexer(), query_document.clone())
    }
}
