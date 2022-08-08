use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use types::jotoba::languages::Language;

/// A single item (result) in a set of search results
#[derive(Clone, Copy, Default, Debug)]
pub struct ResultItem<T> {
    pub item: T,
    pub relevance: usize,
    language: Option<Language>,
}

impl<T: PartialEq> ResultItem<T> {
    /// Create a new ResultItem<T>
    #[inline]
    pub fn new(item: T, relevance: usize) -> Self {
        Self {
            item,
            relevance,
            language: None,
        }
    }
}

impl<T> ResultItem<T> {
    /// Maps the item within the result without changing other data
    pub fn map_item<F, O>(self, f: F) -> ResultItem<O>
    where
        F: Fn(T) -> O,
    {
        let item = (f)(self.item);
        ResultItem {
            item,
            relevance: self.relevance,
            language: self.language,
        }
    }
}

impl<T: PartialEq> PartialEq for ResultItem<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: PartialEq> Eq for ResultItem<T> {}

impl<T: PartialEq + Hash + Eq> Hash for ResultItem<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
    }
}

impl<T: PartialEq> PartialOrd for ResultItem<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.relevance.cmp(&other.relevance))
    }
}

impl<T: PartialEq> Ord for ResultItem<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.relevance.cmp(&other.relevance)
    }
}
