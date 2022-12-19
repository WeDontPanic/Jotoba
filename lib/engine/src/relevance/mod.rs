pub mod data;
pub mod item;

use data::SortData;
use types::jotoba::language::Language;

pub trait RelevanceEngine {
    type OutItem;
    type IndexItem;
    type Query;

    fn init(&mut self, _init: RelEngineInit) {}

    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32;
}

pub struct RelEngineInit {
    pub query: String,
    pub language: Option<Language>,
}

impl RelEngineInit {
    #[inline]
    pub(crate) fn new(query: String, language: Option<Language>) -> Self {
        Self { query, language }
    }
}
