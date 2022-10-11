use engine::relevance::{data::SortData, RelevanceEngine};
use ngindex::{item::IndexItem, termset::TermSet};
use types::jotoba::names::Name;

pub struct ForeignOrder;

impl RelevanceEngine for ForeignOrder {
    type OutItem = &'static Name;
    type IndexItem = IndexItem<u32>;
    type Query = TermSet;

    #[inline]
    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        item.index_item().dice_weighted(item.query(), 0.1)
    }
}
