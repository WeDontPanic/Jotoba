use index_framework::{
    backend::memory::{
        dict::default::Dictionary, postings::compressed::Postings, storage::default::Storage,
        MemBackend,
    },
    retrieve::retriever::default::DefaultRetrieve,
    traits::{backend::Backend, dictionary::IndexDictionary},
};
use sparse_vec::{SpVec32, VecExt};
use types::jotoba::{languages::Language, sentences::Sentence};
use vsm::{dict_term::DictTerm, doc_vec::DocVector};

pub struct Engine {}

impl engine::Engine<'static> for Engine {
    type B = MemBackend<
        DictTerm,
        DocVector<u32>,
        Dictionary<DictTerm>,
        Storage<DocVector<u32>>,
        Postings,
    >;
    type DictItem = DictTerm;
    type Document = DocVector<u32>;
    type Retriever = DefaultRetrieve<'static, Self::B, Self::DictItem, Self::Document>;
    type Output = &'static Sentence;
    type Query = SpVec32;

    fn make_query<S: AsRef<str>>(inp: S, _lang: Option<Language>) -> Option<Self::Query> {
        let query = inp.as_ref();

        let mut terms = all_terms(&query.to_lowercase());
        terms.push(query.to_string().to_lowercase());

        let index = Self::get_index(None);

        let term_ids = terms
            .into_iter()
            .filter_map(|i| index.dict().get_id(&i))
            .map(|id| {
                let term = index.dict().get_term(id).unwrap();
                let weight = term.frequency();
                (id, weight)
            })
            .collect::<Vec<_>>();
        let vec = SpVec32::create_new_raw(term_ids);
        (!vec.is_empty()).then(|| vec)
    }

    #[inline]
    fn doc_to_output(input: &Self::Document) -> Option<Vec<Self::Output>> {
        resources::get()
            .sentences()
            .by_id(*input.document())
            .map(|i| vec![i])
    }

    #[inline]
    fn get_index(_lang: Option<Language>) -> &'static Self::B {
        indexes::get().sentence().foreign()
    }

    #[inline]
    fn retrieve_for(
        inp: &Self::Query,
        _query_str: &str,
        lang: Option<Language>,
    ) -> index_framework::retrieve::Retrieve<'static, Self::B, Self::DictItem, Self::Document> {
        let term_iter = inp.dimensions().map(|i| i as u32);
        Self::retrieve(lang)
            .by_term_ids(term_iter)
            .in_posting(lang.unwrap() as u32)
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
