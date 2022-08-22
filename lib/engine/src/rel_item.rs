use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

/// A single item (result) in a set of search results
#[derive(Clone, Copy, Default, Debug)]
pub struct RelItem<T> {
    pub item: T,
    pub relevance: f32,
}

impl<T: PartialEq> RelItem<T> {
    /// Create a new ResultItem<T>
    #[inline]
    pub fn new(item: T, relevance: f32) -> Self {
        Self { item, relevance }
    }
}

impl<T> RelItem<T> {
    /// Maps the item within the result without changing other data
    #[inline]
    pub fn map_item<F, O>(self, f: F) -> RelItem<O>
    where
        F: Fn(T) -> O,
    {
        let item = (f)(self.item);
        RelItem {
            item,
            relevance: self.relevance,
        }
    }
}

impl<T: PartialEq> PartialEq for RelItem<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: PartialEq> Eq for RelItem<T> {}

impl<T: Eq + Hash> Hash for RelItem<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
    }
}

impl<T: PartialEq> PartialOrd for RelItem<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.relevance.total_cmp(&other.relevance))
    }
}

impl<T: PartialEq> Ord for RelItem<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.relevance.total_cmp(&other.relevance)
    }
}
