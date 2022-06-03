pub mod index;
pub mod regex;
pub mod regex_index;

use crate::engine::{Indexable, SearchEngine};
use resources::storage::ResourceStorage;
use types::jotoba::languages::Language;
use types::jotoba::words::Word;
use vector_space_model2::{DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = u32;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Word;

    #[inline]
    fn doc_to_output<'a>(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<Self::Output>> {
        storage.words().by_sequence(*inp).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let mut terms = vec![(query.to_string(), 1.0)];

        let mut indexer = index.get_indexer().clone();
        for term in tinysegmenter::tokenize(query) {
            let indexed = indexer.find_term(&term)?;
            if indexed.doc_frequency() >= 5_000 || terms.iter().any(|i| i.0 == term) {
                continue;
            }

            terms.push((term, 0.03));
        }

        let vec = index.build_vector_weights(&terms)?;
        Some((vec, query.to_owned()))
    }
}
