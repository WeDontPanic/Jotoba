use std::{cmp::Ordering, collections::HashMap, vec::IntoIter};

use itertools::Itertools;
use resources::parse::jmdict::languages::Language;

/// A structure holding all inforamtion about the results of a search
#[derive(Clone, Default)]
pub(crate) struct SearchResult {
    items: Vec<ResultItem>,
    order_map: HashMap<usize, ResultItem>,
}

/// A single result item for `find`
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct ResultItem {
    pub(crate) seq_id: usize,
    pub(crate) relevance: f32,
    pub(crate) language: Language,
}

impl PartialEq for ResultItem {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.relevance.eq(&other.relevance) && self.seq_id == other.seq_id
    }
}

impl PartialOrd for ResultItem {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relevance
            .partial_cmp(&other.relevance)
            .map(|i| i.reverse())
    }
}

impl Eq for ResultItem {}

impl Ord for ResultItem {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap_or(Ordering::Equal)
    }
}

impl SearchResult {
    /// Creates a new `SearchResult` from items of the results
    #[inline]
    pub(crate) fn new(items: Vec<ResultItem>) -> SearchResult {
        let order_map = Self::build_order_map(&items);
        Self { items, order_map }
    }

    /// Returns a vec of all sequence ids in the results
    #[inline]
    pub(crate) fn sequence_ids(&self) -> Vec<u32> {
        self.items
            .iter()
            .map(|i| i.seq_id as u32)
            .unique()
            .collect()
    }

    /// Returns the searchresults order map
    #[inline]
    pub(crate) fn get_order_map(&self) -> &HashMap<usize, ResultItem> {
        &self.order_map
    }

    /// Converts a SearchResult into a new one with max `limit` items
    #[inline]
    pub(crate) fn get_limit(self, limit: usize) -> Self {
        let items = self.items.into_iter().take(limit).collect::<Vec<_>>();
        let order_map = Self::build_order_map(&items);
        Self { items, order_map }
    }

    /// Builds a HashMap that maps sequence ids to the corresponding ResultItem
    fn build_order_map(items: &[ResultItem]) -> HashMap<usize, ResultItem> {
        let mut order_map: HashMap<usize, ResultItem> = HashMap::new();

        for result_item in items.iter() {
            let entry = order_map
                .entry(result_item.seq_id)
                .or_insert_with(|| *result_item);

            if result_item.relevance > entry.relevance {
                entry.relevance = result_item.relevance;
            }
        }

        order_map
    }
}

impl Extend<ResultItem> for SearchResult {
    #[inline]
    fn extend<T: IntoIterator<Item = ResultItem>>(&mut self, iter: T) {
        self.items.extend(iter);
        self.items.sort_unstable();
        self.items.dedup();
        self.order_map = Self::build_order_map(&self.items);
    }
}

impl IntoIterator for SearchResult {
    type Item = ResultItem;

    type IntoIter = IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
