pub mod k_reading;
pub mod regex;

use crate::engine::{Indexable, SearchEngine};
use indexes::words::NativeIndex;
use japanese::JapaneseExt;
use types::jotoba::{languages::Language, words::Word};
use vector_space_model2::{DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = u32;
    type Index = NativeIndex;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        Some(indexes::get().word().native())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Word;

    #[inline]
    fn doc_to_output<'a>(inp: &Self::Document) -> Option<Vec<Self::Output>> {
        resources::get().words().by_sequence(*inp).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let fmt_query = Self::query_formatted(query);
        let mut terms = vec![(fmt_query.clone(), 1.0)];

        let indexer = index.get_indexer();
        for term in tinysegmenter::tokenize(&fmt_query) {
            let indexed = indexer.find_term(&term)?;
            if indexed.doc_frequency() >= 5_000 || terms.iter().any(|i| i.0 == term) {
                continue;
            }

            terms.push((term, 0.03));
        }

        let vec = index.build_vector_weights(&terms)?;
        Some((vec, fmt_query))
    }

    #[inline]
    fn query_formatted(inp: &str) -> String {
        let q = japanese::to_halfwidth(inp);

        if !inp.has_katakana() && !inp.is_japanese() {
            return q.to_hiragana();
        }

        q
    }
}
