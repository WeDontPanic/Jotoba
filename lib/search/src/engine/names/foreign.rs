use index_framework::{
    retrieve::{retriever::default::DefaultRetrieve, Retrieve},
    traits::{backend::Backend, dictionary::IndexDictionary},
};
use indexes::{names::FOREIGN_NGRAM, words::native::N as NATIVE_NGRAM};
use ngindex2::{item::IndexItem, termset::TermSet, utils::padded, NGIndex, Wordgrams};
use types::jotoba::{languages::Language, names::Name};

pub struct Engine;

impl engine::Engine<'static> for Engine {
    type B = NGIndex<FOREIGN_NGRAM, Self::Document>;
    type DictItem = String;
    type Document = IndexItem<u32>;
    type Retriever = DefaultRetrieve<'static, Self::B, Self::DictItem, Self::Document>;
    // TODO: fix NGramRetriever needing more than `limit` iterations
    //type Retriever = NGramRetriever<'static, NATIVE_NGRAM, Self::B, Self::DictItem, Self::Document>;
    type Output = &'static Name;
    type Query = TermSet;

    fn make_query<S: AsRef<str>>(inp: S, _: Option<Language>) -> Option<Self::Query> {
        let fmt = format_word(inp.as_ref());

        let dict = Self::get_index(None).dict();
        let mut tids: Vec<_> = Wordgrams::new(&padded(&fmt, NATIVE_NGRAM - 1), NATIVE_NGRAM)
            .filter_map(|i| dict.get_id(i))
            .collect();
        tids.sort_unstable();
        println!("{tids:#?}");
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
        indexes::get().name().foreign()
    }

    #[inline]
    fn retrieve_for(
        query: &Self::Query,
        _q_str: &str,
        lang: Option<Language>,
    ) -> Retrieve<'static, Self::B, Self::DictItem, Self::Document> {
        Self::retrieve(lang).by_term_ids(query.iter().copied())
    }
}

#[inline]
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp.to_lowercase());
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out.to_lowercase()
}
