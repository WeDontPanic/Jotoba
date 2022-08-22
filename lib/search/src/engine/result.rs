use std::{fmt::Debug, slice::Iter};

use engine::relevance::item::RelItem;

/// A result from a search. Contains information about the actual
/// amount of items returned and the items to display on the current page.
/// The items are always ordered
pub struct SearchResult<T> {
    pub total_items: usize,
    pub items: Vec<RelItem<T>>,
}

impl<T: PartialEq> SearchResult<T> {
    /// Create a new `SearchResult` from a list of items. Requires `items` to be sorted
    #[inline]
    pub fn new(items: Vec<RelItem<T>>, total_items: usize) -> Self {
        Self { items, total_items }
    }
}

impl<T> SearchResult<T> {
    /// Get the total amount of items in the result. This value is
    /// always bigger or equal to the length of the items in the resultset
    #[inline]
    pub fn len(&self) -> usize {
        self.total_items
    }

    /// Returns `true` if result is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the raw result items
    #[inline]
    pub fn iter(&self) -> Iter<'_, RelItem<T>> {
        self.items.iter()
    }

    #[inline]
    pub fn into_inner(self) -> Vec<RelItem<T>> {
        self.items
    }

    /// Returns an iterator over the raw result items
    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.items.into_iter().map(|i| i.item)
    }

    /// Returns the item at `index` from the result or None if index is out of bounds
    #[inline]
    pub fn get(&self, index: usize) -> Option<&RelItem<T>> {
        self.items.get(index)
    }
}

impl<T: PartialEq> Default for SearchResult<T> {
    #[inline]
    fn default() -> Self {
        Self {
            total_items: 0,
            items: vec![],
        }
    }
}

impl<T: PartialEq + Debug> Debug for SearchResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchResult")
            .field("total_items", &self.total_items)
            .field("items", &self.items)
            .finish()
    }
}
