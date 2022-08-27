pub mod k_reading;
pub mod regex;

use crate::engine::{Indexable, SearchEngine};
use index_framework::{
    retrieve::retriever::default::DefaultRetrieve,
    traits::{backend::Backend, dictionary::IndexDictionary},
};
use indexes::words::{NativeIndex, NATIVE_NGRAM};
use japanese::JapaneseExt;
use ngindex2::{item::IndexItem, termset::TermSet, utils::padded, NGIndex, Wordgrams};
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

pub struct Engine2 {}

impl engine::Engine<'static> for Engine2 {
    type B = NGIndex<NATIVE_NGRAM, Self::Document>;
    type DictItem = String;
    type Document = IndexItem<u32>;
    type Retriever = DefaultRetrieve<'static, Self::B, Self::DictItem, Self::Document>;
    // TODO: fix NGramRetriever needing more than `limit` iterations
    //type Retriever = NGramRetriever<'static, NATIVE_NGRAM, Self::B, Self::DictItem, Self::Document>;
    type Output = &'static Word;
    type Query = TermSet;

    fn make_query<S: AsRef<str>>(inp: S, _: Option<Language>) -> Option<Self::Query> {
        let dict = Self::get_index(None).dict();
        let mut tids: Vec<_> =
            Wordgrams::new(&padded(inp.as_ref(), NATIVE_NGRAM - 1), NATIVE_NGRAM)
                .filter_map(|i| dict.get_id(i))
                .collect();
        tids.sort_unstable();
        if tids.is_empty() {
            return None;
        }
        Some(TermSet::new(tids))
    }

    #[inline]
    fn doc_to_output(input: &Self::Document) -> Option<Vec<Self::Output>> {
        resources::get()
            .words()
            .by_sequence(*input.item())
            .map(|i| vec![i])
    }

    #[inline]
    fn get_index(_: Option<Language>) -> &'static Self::B {
        indexes::get().word().native2()
    }

    #[inline]
    fn retrieve_for(
        query: &Self::Query,
        _q_str: &str,
        lang: Option<Language>,
    ) -> index_framework::retrieve::Retrieve<'static, Self::B, Self::DictItem, Self::Document> {
        Self::retrieve(lang).by_term_ids(query.iter().copied())
    }
}
