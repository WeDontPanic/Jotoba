use std::{cmp::min, collections::BinaryHeap, vec::IntoIter};

use super::result_item::ResultItem;

/// A result from a search. Contains information about the actual amount of items returned and the
/// limited items to display
pub struct SearchResult<T: PartialEq> {
    pub total_items: usize,
    pub items: Vec<ResultItem<T>>,
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

impl<T: PartialEq> SearchResult<T> {
    /// Returns a `SearchResult` from a BinaryHeap
    pub(crate) fn from_binary_heap(
        mut heap: BinaryHeap<ResultItem<T>>,
        offset: usize,
        limit: usize,
    ) -> Self {
        let total_items = heap.len();

        if offset >= heap.len() {
            return Self::default();
        }
        let item_count = min(heap.len() - offset, limit);
        let mut items = Vec::with_capacity(item_count);

        for _ in 0..offset {
            heap.pop();
        }

        for _ in 0..item_count {
            items.push(heap.pop().unwrap());
        }

        Self { total_items, items }
    }

    /// Get the amount of items in the result
    #[inline]
    pub fn len(&self) -> usize {
        self.total_items
    }

    /// Returns an iterator over the raw result items
    #[inline]
    pub fn item_iter(self) -> impl Iterator<Item = T> {
        self.items.into_iter().map(|i| i.item)
    }
}

impl<T: PartialEq> IntoIterator for SearchResult<T> {
    type Item = ResultItem<T>;

    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
