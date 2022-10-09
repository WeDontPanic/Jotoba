use index_framework::retrieve::retriever::default::DefaultRetrieve;
use types::jotoba::languages::Language;
use types::jotoba::words::Word;

pub struct Engine;

impl engine::Engine<'static> for Engine {
    type B = indexes::kanji::reading::Index;
    type DictItem = String;
    type Document = u32;
    type Retriever = DefaultRetrieve<'static, Self::B, Self::DictItem, Self::Document>;
    type Output = &'static Word;
    type Query = String;

    fn make_query<S: AsRef<str>>(inp: S, _lang: Option<Language>) -> Option<Self::Query> {
        Some(inp.as_ref().to_string())
    }

    #[inline]
    fn doc_to_output(input: &Self::Document) -> Option<Vec<Self::Output>> {
        resources::get()
            .words()
            .by_sequence(*input)
            .map(|i| vec![i])
    }

    #[inline]
    fn get_index(_lang: Option<Language>) -> &'static Self::B {
        indexes::get().word().k_reading()
    }

    fn retrieve_for(
        inp: &Self::Query,
        _query_str: &str,
        _lang: Option<Language>,
    ) -> index_framework::retrieve::Retrieve<'static, Self::B, Self::DictItem, Self::Document> {
        Self::retrieve(None).by_term(inp)
    }
}
