pub mod pushable;
pub mod rel_item;
pub mod relevance;
pub mod result;
pub mod task;
pub mod utils;

use index_framework::{
    retrieve::{retriever::Retriever, Retrieve},
    traits::{backend::Backend, deser::DeSer},
};
use std::hash::Hash;
use types::jotoba::languages::Language;

/// Generic search engine
pub trait Engine<'index> {
    // Index
    type B: Backend<Self::DictItem, Self::Document>;

    // Index dictionary term
    type DictItem: DeSer + Ord + From<String>;

    /// Index output
    type Document: DeSer;

    /// Retrieving algorithm
    type Retriever: Retriever<
        'index,
        Self::B,
        Self::DictItem,
        Self::Document,
        Output = Self::Document,
    >;

    /// Engine output
    type Output: Eq + Hash + Clone;

    /// The search query
    type Query: Into<Self::DictItem> + Clone;

    fn make_query<S: AsRef<str>>(inp: S, lang: Option<Language>) -> Option<Self::Query>;

    /// Converts index output to engine output
    fn doc_to_output(input: &Self::Document) -> Option<Vec<Self::Output>>;

    /// Returns the engines index
    fn get_index(lang: Option<Language>) -> &'index Self::B;

    /// Returns a new retrieve for the engine
    #[inline]
    fn retrieve(
        lang: Option<Language>,
    ) -> Retrieve<'index, Self::B, Self::DictItem, Self::Document> {
        Retrieve::new(Self::get_index(lang))
    }

    /// Returns a new retrieve for the given terms
    #[inline]
    fn retrieve_for(
        inp: Self::Query,
        lang: Option<Language>,
    ) -> Retrieve<'index, Self::B, Self::DictItem, Self::Document> {
        let di: Self::DictItem = inp.into();
        Retrieve::new(Self::get_index(lang)).by_term(di)
    }
}
