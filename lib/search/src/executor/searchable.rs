use super::{
    out_builder::{OutputAddable, OutputBuilder},
    producer::Producer,
};
use crate::query::Query;
use std::{fmt::Debug, hash::Hash};

pub trait Searchable {
    type Item: Clone + Eq + Hash + Debug;
    type OutItem;
    type ResAdd: OutputAddable;

    fn get_producer<'s>(&'s self) -> &Vec<Box<dyn Producer<Target = Self> + 's>>;

    fn get_query(&self) -> &Query;

    /// Converts from `Self::Item` to `Self::OutputItem`
    fn to_output_item(&self, item: Self::Item) -> Self::OutItem;

    /// Allows modifying the collected producers output before converting it to a SearchResult
    fn mod_output(&self, _out: &mut OutputBuilder<Self::Item, Self::ResAdd>) {}

    /// Should return `true` if the passed item should be ignored / filtered
    fn filter(&self, _item: &Self::Item) -> bool {
        false
    }

    #[inline]
    fn max_top_dist(&self) -> Option<f32> {
        None
    }
}
