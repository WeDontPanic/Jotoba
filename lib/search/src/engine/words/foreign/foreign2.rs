use index_framework::{
    backend::memory::{
        dict::default::Dictionary, postings::compressed::Postings, storage::default::Storage,
        MemBackend,
    },
    retrieve::{retriever::default::DefaultRetrieve, Retrieve},
    traits::{backend::Backend, dictionary::IndexDictionary},
};

use types::jotoba::{languages::Language, words::Word};
use vsm::{dict_term::DictTerm, doc_vec::DocVector, Vector};

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
    type Output = &'static Word;
    type Query = Vector;

    fn make_query<S: AsRef<str>>(inp: S, lang: Option<Language>) -> Option<Self::Query> {
        let dict = Self::get_index(lang).dict();

        let sparse = inp
            .as_ref()
            .split(' ')
            .filter_map(|term| dict.get_id(term))
            .map(|i| (i, 0.001))
            .chain(dict.get_id(inp.as_ref()).map(|i| (i, 1.0)));

        let vec = Vector::create_new_raw(sparse);

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
        let term_iter = query.sparse().iter().map(|i| i.0);
        Self::retrieve(lang).by_term_ids(term_iter)
    }
}
