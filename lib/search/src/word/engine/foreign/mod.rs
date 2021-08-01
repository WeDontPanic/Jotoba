mod gen;
pub(super) mod index;
mod metadata;

use self::index::Index;
use super::{
    document::Document,
    result::{ResultItem, SearchResult},
};
use crate::query::Query;
use error::Error;
use gen::GenDoc;
use parse::jmdict::languages::Language;
use std::cmp::Ordering;
use vector_space_model::{document_vector, DocumentVector};

pub(crate) struct Find<'a> {
    limit: usize,
    offset: usize,
    query: &'a Query,
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

        let res = self.vecs_to_result_items(&query_vec, document_vectors, language);

        Ok(res)
    }

    /// Converts document vectors to result items
    fn vecs_to_result_items(
        &self,
        query_vec: &DocumentVector<GenDoc>,
        document_vectors: Vec<DocumentVector<Document>>,
        language: Language,
    ) -> Vec<ResultItem> {
        // Sort by relevance
        let mut found: Vec<_> = document_vectors
            .iter()
            .filter_map(|i| {
                let similarity = i.similarity(query_vec);
                (similarity != 0f32).then(|| (i, similarity))
            })
            .collect();

        // Sort by similarity to top
        found.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal).reverse());
        found.dedup_by(|a, b| a.0.document == b.0.document);

        // Convert DocumentVectors to ResultItems
        found
            .into_iter()
            .map(|i| i.0.document.seq_ids.iter().copied().map(move |j| (j, i.1)))
            .skip(self.offset)
            .take(self.limit)
            .flatten()
            .map(|i| ResultItem {
                seq_id: i.0,
                relevance: i.1,
                language,
            })
            .collect::<Vec<_>>()
    }

    #[inline]
    fn get_query_str(&self) -> &str {
        &self.query.query
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
