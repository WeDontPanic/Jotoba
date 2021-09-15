mod gen;
pub(crate) mod index;
mod metadata;

use self::index::Index;
use crate::{
    engine::{
        document::MultiDocument,
        result::{ResultItem, SearchResult},
        FindExt,
    },
    query::Query,
};
use error::Error;
use gen::GenDoc;
use resources::parse::jmdict::languages::Language;
use vector_space_model::{document_vector, DocumentVector};

pub(crate) struct Find<'a> {
    limit: usize,
    offset: usize,
    query: &'a Query,
}

impl<'a> FindExt for Find<'a> {
    type ResultItem = ResultItem;
    type GenDoc = gen::GenDoc;
    type Document = MultiDocument;

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
        &self.query.query
    }
}

impl<'a> Find<'a> {
    #[inline]
    pub(crate) fn new(query: &'a Query, limit: usize, offset: usize) -> Self {
        Self {
            limit,
            offset,
            query,
        }
    }

    /// Do a foreign word search
    pub(crate) async fn find(&self) -> Result<SearchResult, Error> {
        let language = self.query.settings.user_lang;

        let native_results = self.find_lang(language).await?;
        if self.get_lang() == Language::English {
            return Ok(SearchResult::new(native_results));
        }

        let english_results = self.find_lang(Language::English).await?;

        // Chain custom and english results
        let mut result = native_results
            .into_iter()
            .chain(english_results.into_iter())
            .collect::<Vec<_>>();

        result.sort();
        result.dedup();

        Ok(SearchResult::new(result))
    }

    /// Find results for a given language
    async fn find_lang(&self, language: Language) -> Result<Vec<ResultItem>, Error> {
        let index = match index::get(language) {
            Some(index) => index,
            None => return Ok(vec![]),
        };

        let query_vec = match self.gen_query_vec(&index) {
            Some(vec) => vec,
            None => return Ok(vec![]),
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

        let result = self
            .vecs_to_result_items(&query_vec, &document_vectors)
            .into_iter()
            .map(|i| {
                let rel = i.relevance;
                i.document.seq_ids.iter().map(move |j| (*j, rel))
            })
            .flatten()
            .map(|(seq_id, rel)| ResultItem {
                seq_id: seq_id as usize,
                relevance: rel,
                language,
            })
            .collect();

        Ok(result)
    }

    #[inline]
    fn get_lang(&self) -> Language {
        self.query.settings.user_lang
    }

    /// Generates a `DocumentVector<GenDoc>` for the given query
    fn gen_query_vec(&self, index: &Index) -> Option<DocumentVector<GenDoc>> {
        let query_str = self.get_query_str();

        let term_indexer = index.get_indexer();
        let doc_store = index.get_vector_store();

        // search query to document vector
        let query_document = GenDoc::new(query_str, vec![]);
        let mut query = document_vector::DocumentVector::new(term_indexer, query_document.clone())?;

        let result_count = query
            .vector()
            .vec_indices()
            .map(|dim| doc_store.dimension_size(dim))
            .sum::<usize>();

        if result_count < 15 {
            // Add substrings of query to query document vector
            let sub_terms: Vec<_> = GenDoc::sub_documents(&query_document)
                .into_iter()
                .map(|i| document_vector::Document::get_terms(&i))
                .flatten()
                .collect();

            query.add_terms(term_indexer, &sub_terms, true, Some(0.3));
        }

        Some(query)
    }
}
