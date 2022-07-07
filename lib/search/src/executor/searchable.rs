use super::{out_builder::OutputBuilder, producer::Producer};
use crate::query::Query;
use std::hash::Hash;

pub trait Searchable {
    type OutputAdd: Default;
    type OutputItem;
    type Item: Clone + Eq + Hash;

    fn get_producer<'s>(&'s self) -> &Vec<Box<dyn Producer<Target = Self> + 's>>;

    fn get_query(&self) -> &Query;

    /// Converts from `Self::Item` to `Self::OutputItem`
    fn to_output_item(&self, item: Self::Item) -> Self::OutputItem;

    /// Allows modifying the collected producers output before converting it to a SearchResult
    fn mod_output(&self, _out: &mut OutputBuilder<Self::Item, Self::OutputAdd>) {}

    /// Should return `true` if the passed item should be ignored / filtered
    fn filter(&self, _item: &Self::Item) -> bool {
        false
    }
}
