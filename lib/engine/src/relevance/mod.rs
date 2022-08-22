pub mod data;
pub mod item;

use data::SortData;

pub trait RelevanceEngine {
    type OutItem;
    type IndexItem;
    type Query;

    fn score<'item, 'query>(
        &mut self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32;
}
