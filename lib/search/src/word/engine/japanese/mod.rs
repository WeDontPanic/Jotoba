mod document;
pub(crate) mod gen;
pub(super) mod index;
pub(crate) mod kanji;

use self::{gen::GenDoc, index::Index};

use super::{
    result::{ResultItem, SearchResult},
    CmpDocument, FindExt,
};
use error::Error;
use resources::parse::jmdict::languages::Language;
use vector_space_model::DocumentVector;

pub(crate) struct Find<'a> {
    limit: usize,
    offset: usize,
    query: &'a str,
}

impl<'a> Find<'a> {
    #[inline]
    pub(crate) fn new(query: &'a str, limit: usize, offset: usize) -> Self {
        Self {
            limit,
            offset,
            query,
        }
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
        let document_vectors = doc_store
            .get_all_async(&dimensions)
            .await
            .map_err(|_| error::Error::NotFound)?;

        let sort = |a: &CmpDocument<_>, b: &CmpDocument<_>| {
            let a_rev = (a.relevance * 1000f32) as u32;
            let b_rev = (b.relevance * 1000f32) as u32;
            a_rev.cmp(&b_rev).reverse()
        };

        let items = self
            .vecs_to_result_items(&query_vec, &document_vectors, sort)
            .into_iter()
            .map(|doc| ResultItem {
                seq_id: doc.document.seq_id as usize,
                relevance: doc.relevance,
                language: Language::English,
            })
            .collect::<Vec<_>>();

        Ok(SearchResult::new(items))
    }
    /// Generate a document vector out of `query_str`
    #[inline]
    fn gen_query(&self, index: &Index) -> Option<DocumentVector<GenDoc>> {
        let query_document = GenDoc::new(vec![self.query.to_string()], 0);
        let mut doc = DocumentVector::new(index.get_indexer(), query_document.clone())?;

        // TODO: look if this makes the results really better. If not, remove
        let terms = tinysegmenter::tokenize(self.query);
        doc.add_terms(index.get_indexer(), &terms, true, Some(0.03));

        Some(doc)
    }
}

/// Returns the loaded japanese index
#[inline]
pub(crate) fn get_index() -> &'static Index {
    index::INDEX.get().unwrap()
}

impl<'a> FindExt for Find<'a> {
    type ResultItem = super::result::ResultItem;
    type GenDoc = gen::GenDoc;
    type Document = document::Document;

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
