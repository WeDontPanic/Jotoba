use super::result_item::ResultItem;
use std::{collections::HashSet, fmt::Debug, hash::Hash, ops::Index, slice::Iter, vec::IntoIter};

/// A result from a search. Contains information about the actual
/// amount of items returned and the items to display on the current page.
/// The items are always ordered
pub struct SearchResult<T> {
    pub total_items: usize,
    pub items: Vec<ResultItem<T>>,
}

impl<T: PartialEq> SearchResult<T> {
    /// Create a new `SearchResult` from a list of items. Requires `items` to be sorted
    #[inline]
    pub fn new(items: Vec<ResultItem<T>>, total_items: usize) -> Self {
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
    pub fn iter(&self) -> Iter<'_, ResultItem<T>> {
        self.items.iter()
    }

    #[inline]
    pub fn into_inner(self) -> Vec<ResultItem<T>> {
        self.items
    }

    /// Returns an iterator over the raw result items
    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.items.into_iter().map(|i| i.item)
    }
}

impl<T: Clone + Hash + Eq> SearchResult<T> {
    pub fn merge<O>(&mut self, other: O)
    where
        O: Iterator<Item = ResultItem<T>>,
    {
        self.total_items += merge_sorted_list(&mut self.items, other);
    }
}

/// Merges two sorted sequences `other` and `src` and stores result into `src`. Ignores duplicates.
fn merge_sorted_list<O, T: Clone + Hash + Eq>(src: &mut Vec<ResultItem<T>>, other: O) -> usize
where
    O: Iterator<Item = ResultItem<T>>,
{
    let mut add_cnt = 0;

    // Use a hashset to be able to look up whether an element from `other` is already in `src` the
    // fastest way possible
    let hash_set = HashSet::<T>::from_iter(src.clone().into_iter().map(|i| i.item));

    for i in other {
        if !hash_set.contains(&i.item) {
            add_cnt += 1;
            src.push(i);
        }
    }

    // We might have changed the ordering
    src.sort_by(|a, b| a.cmp(&b).reverse());
    add_cnt
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
