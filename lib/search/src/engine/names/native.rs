use index_framework::{
    retrieve::{retriever::default::DefaultRetrieve, Retrieve},
    traits::{backend::Backend, dictionary::IndexDictionary},
};
use indexes::words::native::N as NATIVE_NGRAM;
use jp_utils::JapaneseExt;
use ngindex::{item::IndexItem, termset::TermSet, utils::padded, NGIndex, Wordgrams};
use types::jotoba::{language::Language, names::Name};

pub struct Engine;

impl engine::Engine<'static> for Engine {
    type B = NGIndex<NATIVE_NGRAM, Self::Document>;
    type DictItem = String;
    type Document = IndexItem<u32>;
    type Retriever = DefaultRetrieve<'static, Self::B, Self::DictItem, Self::Document>;
    // TODO: fix NGramRetriever needing more than `limit` iterations
    //type Retriever = NGramRetriever<'static, NATIVE_NGRAM, Self::B, Self::DictItem, Self::Document>;
    type Output = &'static Name;
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
            .names()
            .by_sequence(*input.item())
            .map(|i| vec![i])
    }

    #[inline]
    fn get_index(_: Option<Language>) -> &'static Self::B {
        indexes::get().name().native()
    }

    #[inline]
    fn retrieve_for(
        query: &Self::Query,
        _q_str: &str,
        lang: Option<Language>,
    ) -> Retrieve<'static, Self::B, Self::DictItem, Self::Document> {
        let search_in;

        if _q_str.is_kanji() {
            search_in = 2;
        } else if _q_str.has_kanji() {
            search_in = 1;
        } else {
            search_in = 0;
        }

        Self::retrieve(lang)
            .by_term_ids(query.iter().copied())
            .in_posting(search_in)
    }
}
