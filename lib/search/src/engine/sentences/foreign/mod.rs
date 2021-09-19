use error::Error;
use resources::parse::jmdict::languages::Language;
use vector_space_model::DocumentVector;

use crate::engine::{
    document::SentenceDocument,
    result::{ResultItem, SearchResult},
    simple_gen_doc::GenDoc,
    FindExt,
};

use self::index::Index;

pub(crate) mod index;

pub(crate) struct Find<'a> {
    limit: usize,
    offset: usize,
    query: &'a str,
    language: Language,
    in_english: bool,
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
    pub(crate) fn new(query: &'a str, language: Language, limit: usize, offset: usize) -> Self {
        Self {
            limit,
            offset,
            query,
            language,
            in_english: false,
        }
    }

    /// Do a foreign word search
    pub(crate) async fn find(&self) -> Result<SearchResult, Error> {
        let index = index::get(self.language).ok_or(Error::Unexpected)?;

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
        let index = index::get(self.language).ok_or(Error::Unexpected)?;

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
            .vecs_to_result_items(&query_vec, &document_vectors, 0.05f32)
            .into_iter()
            .map(|i| ResultItem {
                seq_id: i.document.seq_id,
                relevance: i.relevance,
                language: Language::English,
            })
            .collect();

        Ok(SearchResult::new(result))
    }

    /// Generate a document vector out of `query_str`
    fn gen_query(&self, index: &Index) -> Option<DocumentVector<GenDoc>> {
        let mut terms = all_terms(&self.query.to_lowercase());
        terms.push(self.query.to_string().to_lowercase());
        let query_document = GenDoc::new(terms);
        DocumentVector::new(index.get_indexer(), query_document.clone())
    }
}

/// Splits a string into all its terms.
///
/// # Example
/// "make some coffee" => vec!["make","some","coffee"];
pub(crate) fn all_terms(i: &str) -> Vec<String> {
    i.split(' ')
        .map(|i| {
            format_word(i)
                .split(' ')
                .map(|i| i.to_lowercase())
                .filter(|i| !i.is_empty())
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

/// Replaces all special characters into spaces so we can split it down into words
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp);
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out
}
