use std::{
    cmp::min,
    collections::{BinaryHeap, HashSet},
    fmt::Debug,
    hash::Hash,
    ops::Index,
    time::Instant,
    vec::IntoIter,
};

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

impl<T: PartialEq + Debug> Debug for SearchResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchResult")
            .field("total_items", &self.total_items)
            .field("items", &self.items)
            .finish()
    }
}

impl<T: PartialEq + Hash + Clone> SearchResult<T> {
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

    /// Returns `true` if result is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the raw result items
    #[inline]
    pub fn item_iter(self) -> impl Iterator<Item = T> {
        self.items.into_iter().map(|i| i.item)
    }

    pub fn merge(&mut self, other: Self) {
        let start = Instant::now();
        merge_sorted_list(&mut self.items, other.items);
        println!("merging took: {:?}", start.elapsed());
    }
}

/// Merges two sorted sequences `other` and `src` and stores result into `src`. Ignores duplicates.
fn merge_sorted_list<T: PartialEq + Clone + Hash>(
    src: &mut Vec<ResultItem<T>>,
    other: Vec<ResultItem<T>>,
) {
    let mut hash_set = HashSet::with_capacity(src.len() + other.len());
    hash_set.extend(src.clone());
    hash_set.extend(other.clone());

    // TODO: maybe make this O(n) since both sets are already sorted
    src.truncate(0);
    src.extend(hash_set);

    src.sort_unstable_by(|a, b| a.cmp(&b).reverse());
    src.dedup_by(|a, b| a.item.eq(&b.item));
}

impl<T: PartialEq> IntoIterator for SearchResult<T> {
    type Item = ResultItem<T>;
    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T: PartialEq> Index<usize> for SearchResult<T> {
    type Output = ResultItem<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}
