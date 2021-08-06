pub mod document;
pub(crate) mod foreign;
pub(crate) mod japanese;
pub mod result;

use std::{cmp::Ordering, error};

use config::Config;
use vector_space_model::{document_vector, DocumentVector};

use self::document::Document;

/// Load all indexes for word search engine
pub fn load_indexes(config: &Config) -> Result<(), Box<dyn error::Error>> {
    foreign::index::load(config)?;
    japanese::index::load(config);
    Ok(())
}

pub(crate) trait FindExt {
    type ResultItem;
    type GenDoc: document_vector::Document;

    fn get_limit(&self) -> usize;
    fn get_offset(&self) -> usize;
    fn get_query_str(&self) -> &str;

    /// Converts document vectors to result items
    fn vecs_to_result_items<F, S>(
        &self,
        query_vec: &DocumentVector<Self::GenDoc>,
        document_vectors: Vec<DocumentVector<Document>>,
        mut res_map: F,
        mut sort_fn: S,
    ) -> Vec<Self::ResultItem>
    where
        F: FnMut(usize, f32) -> Self::ResultItem,
        S: FnMut(&(&DocumentVector<Document>, f32), &(&DocumentVector<Document>, f32)) -> Ordering,
    {
        // Sort by relevance
        let mut found: Vec<_> = document_vectors
            .iter()
            .filter_map(|i| {
                let similarity = i.similarity(query_vec);
                (similarity != 0f32).then(|| (i, similarity))
            })
            .collect();

        // Sort by similarity to top
        found.sort_by(|a, b| sort_fn(a, b));
        found.dedup_by(|a, b| a.0.document == b.0.document);

        // Convert DocumentVectors to ResultItems
        found
            .into_iter()
            .map(|i| i.0.document.seq_ids.iter().copied().map(move |j| (j, i.1)))
            .skip(self.get_offset())
            .take(self.get_limit())
            .flatten()
            .map(|i| res_map(i.0, i.1))
            .collect::<Vec<_>>()
    }
}
