use index_framework::{
    backend::memory::{
        dict::default::Dictionary, postings::compressed::Postings, storage::default::Storage,
        MemBackend,
    },
    retrieve::retriever::default::DefaultRetrieve,
    traits::{backend::Backend, dictionary::IndexDictionary},
};
use jp_utils::JapaneseExt;
use sentence_reader::output::ParseResult;
use sparse_vec::{SpVec32, VecExt};
use std::collections::HashSet;
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
        let mut terms: HashSet<String> = HashSet::new();

        let dict = Self::get_index(None).dict();

        let query = inp.as_ref();

        if dict.get_id(query).is_some() {
            terms.insert(query.to_string());
        } else {
            match sentence_reader::Parser::new(query).parse() {
                ParseResult::Sentence(s) => {
                    terms.extend(s.iter().map(|i| i.get_inflected()));
                    terms.extend(s.iter().map(|i| i.get_normalized()));
                }
                ParseResult::InflectedWord(w) => {
                    let infl = w.get_inflected();
                    //println!("inflected: {infl:?}: {:?}", dict.get_id(&infl));
                    if dict.get_id(&infl).is_some() {
                        terms.insert(infl);
                    } else {
                        terms.insert(w.get_normalized());
                    }
                }
                ParseResult::None => (),
            };
        }

        //terms.retain(|w| !index.is_stopword_cust(&w, 10.0).unwrap_or(true));

        let terms = terms.into_iter().map(|i| format_query(&i)).filter_map(|i| {
            let id = dict.get_id(&i);
            //let term = dict.get_term(id).unwrap();
            Some((id?, 1.0))
        });

        let vec = SpVec32::create_new_raw(terms);
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
        indexes::get().sentence().native()
    }

    #[inline]
    fn retrieve_for(
        inp: &Self::Query,
        _query_str: &str,
        lang: Option<Language>,
    ) -> index_framework::retrieve::Retrieve<'static, Self::B, Self::DictItem, Self::Document> {
        let term_iter = inp.dimensions().map(|i| i as u32);
        if let Some(lang) = lang {
            Self::retrieve(Some(lang))
                .by_term_ids(term_iter)
                .in_posting(lang as u32)
        } else {
            let langs = Language::iter_word().map(|i| i as u32);
            Self::retrieve(None)
                .by_term_ids(term_iter)
                .in_postings(langs)
        }
    }
}

#[inline]
fn format_query(inp: &str) -> String {
    inp.to_halfwidth()
}
