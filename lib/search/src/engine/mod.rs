pub(crate) mod document;
pub(crate) mod name;
pub mod result;
pub(crate) mod sentences;
pub(crate) mod simple_gen_doc;
pub(crate) mod word;

use std::{cmp::Ordering, error};

use config::Config;
use vector_space_model::{document_vector, traits::Decodable, DocumentVector};

/// Load all indexes for word search engine
pub fn load_indexes(config: &Config) -> Result<(), Box<dyn error::Error>> {
    word::foreign::index::load(config)?;
    word::japanese::index::load(config);
    name::japanese::index::load(config);
    name::foreign::index::load(config);
    Ok(())
}

/// A `Document` wrapping structure where the document has been compared to a given query. The
/// `relevance` field indicates the relevance compared to the query
#[derive(Debug)]
pub(crate) struct CmpDocument<'a, T: Decodable> {
    relevance: f32,
    document: &'a T,
}

impl<'a, T: Decodable + Clone + Copy> Copy for CmpDocument<'a, T> {}

impl<'a, T: Decodable + Clone + Copy> Clone for CmpDocument<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        CmpDocument {
            document: self.document,
            relevance: self.relevance,
        }
    }
}

impl<'a, T: Decodable> CmpDocument<'a, T> {
    #[inline]
    fn new(document: &'a T, relevance: f32) -> Self {
        Self {
            document,
            relevance,
        }
    }
}

pub(crate) trait FindExt {
    type ResultItem;
    type GenDoc: document_vector::Document;
    type Document: Decodable + Eq;

    fn get_limit(&self) -> usize;
    fn get_offset(&self) -> usize;
    fn get_query_str(&self) -> &str;

    /// Converts document vectors to result items
    fn vecs_to_result_items<'a, S>(
        &self,
        query_vec: &DocumentVector<Self::GenDoc>,
        document_vectors: &'a Vec<DocumentVector<Self::Document>>,
        mut sort_fn: S,
    ) -> Vec<CmpDocument<'a, Self::Document>>
    where
        S: FnMut(&CmpDocument<Self::Document>, &CmpDocument<Self::Document>) -> Ordering,
    {
        // Sort by relevance
        let mut found: Vec<_> = document_vectors
            .iter()
            .filter_map(|i| {
                let similarity = i.similarity(query_vec);
                (similarity != 0f32).then(|| CmpDocument::new(&i.document, similarity))
            })
            .collect();

        // Sort by similarity to top
        found.sort_by(|a, b| sort_fn(a, b));
        found.dedup_by(|a, b| a.document == b.document);

        // Convert DocumentVectors to ResultItems
        found
            .into_iter()
            .skip(self.get_offset())
            .take(self.get_limit())
            .collect::<Vec<_>>()
    }
}
