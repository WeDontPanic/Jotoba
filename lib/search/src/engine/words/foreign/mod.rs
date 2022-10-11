use index_framework::{
    backend::memory::{
        dict::default::Dictionary, postings::compressed::Postings, storage::default::Storage,
        MemBackend,
    },
    retrieve::{retriever::default::DefaultRetrieve, Retrieve},
    traits::{backend::Backend, dictionary::IndexDictionary},
};

use once_cell::sync::Lazy;
use regex::Regex;
use sparse_vec::{SpVec32, VecExt};
use types::jotoba::{languages::Language, words::Word};
use vsm::{dict_term::DictTerm, doc_vec::DocVector};

pub struct Engine {}

const FORMAT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("^to ").unwrap());

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
    type Output = &'static Word;
    type Query = SpVec32;

    fn make_query<S: AsRef<str>>(inp: S, lang: Option<Language>) -> Option<Self::Query> {
        let dict = Self::get_index(lang).dict();

        let inp = inp.as_ref().trim().to_lowercase();
        let inp = FORMAT_REGEX.replace_all(&inp, "").to_string();

        let add_term_iter = inp
            .split(' ')
            .filter_map(|term| dict.get_id(term))
            .map(|i| (i, 0.001));

        let sparse = dict
            .get_id(&inp)
            .map(|i| (i, 1.0))
            .into_iter()
            .chain(add_term_iter);

        let vec = SpVec32::create_new_raw(sparse);

        if vec.is_empty() {
            return None;
        }

        Some(vec)
    }

    #[inline]
    fn doc_to_output(input: &Self::Document) -> Option<Vec<Self::Output>> {
        resources::get()
            .words()
            .by_sequence(*input.document())
            .map(|i| vec![i])
    }

    #[inline]
    fn get_index(lang: Option<Language>) -> &'static Self::B {
        indexes::get().word().foreign(lang.unwrap()).unwrap()
    }

    #[inline]
    fn retrieve_for(
        query: &Self::Query,
        _q_str: &str,
        lang: Option<Language>,
    ) -> Retrieve<'static, Self::B, Self::DictItem, Self::Document> {
        let term_iter = query.as_vec().iter().map(|i| i.0);
        Self::retrieve(lang).by_term_ids(term_iter)
    }
}
