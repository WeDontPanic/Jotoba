mod gen;
pub(super) mod index;

use std::cmp::Ordering;

use self::{gen::GenDoc, index::Index};

use super::{
    document::Document,
    result::{ResultItem, SearchResult},
    FindExt,
};
use error::Error;
use parse::jmdict::languages::Language;
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

        // VecStore is surrounded by an Arc
        let mut doc_store = index.get_vector_store().clone();

        // All vectors in queries dimensions
        let dimensions = query_vec.vector().vec_indices().collect::<Vec<_>>();

        // Retrieve all matching vectors
        let document_vectors = doc_store
            .get_all_async(&dimensions)
            .await
            .map_err(|_| error::Error::NotFound)?;

        let res_item_map = |seq, rel| ResultItem {
            seq_id: seq,
            relevance: rel,
            language: Language::English,
        };

        let sort = |a: &(&DocumentVector<Document>, f32), b: &(&DocumentVector<Document>, f32)| {
            let mut a_rev = (a.1 * 1000f32) as u32;
            let mut b_rev = (b.1 * 1000f32) as u32;

            let a_len = a.0.document.len.unwrap();
            let b_len = b.0.document.len.unwrap();
            a_rev -= (a_len * 2) as u32;
            b_rev -= (b_len * 2) as u32;

            a_rev.cmp(&b_rev).reverse()
        };

        let items = self.vecs_to_result_items(&query_vec, document_vectors, res_item_map, sort);
        Ok(SearchResult::new(items))
    }

    /// Generate a document vector out of `query_str`
    #[inline]
    fn gen_query(&self, index: &Index) -> Option<DocumentVector<GenDoc>> {
        let query_document = GenDoc::new(vec![self.query.to_string()], vec![]);
        let doc = DocumentVector::new(index.get_indexer(), query_document.clone())?;
        Some(doc)
    }
}

impl<'a> FindExt for Find<'a> {
    type ResultItem = super::result::ResultItem;
    type GenDoc = gen::GenDoc;

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
