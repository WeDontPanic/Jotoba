use std::{cmp::Ordering, collections::HashMap};

use parse::jmdict::languages::Language;

/// A structure holding all inforamtion about the results of a search
#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    items: Vec<ResultItem>,
    order_map: HashMap<usize, ResultItem>,
}

/// A single result item for `find`
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ResultItem {
    pub(crate) seq_id: usize,
    pub(crate) relevance: f32,
    pub(crate) language: Language,
}

impl PartialEq for ResultItem {
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
    pub(crate) fn sequence_ids(&self) -> Vec<i32> {
        self.items.iter().map(|i| i.seq_id as i32).collect()
        //self.order_map.iter().map(|(k, _)| *k as i32).collect()
    }

    /// Returns the searchresults order map
    #[inline]
    pub(crate) fn get_order_map(&self) -> &HashMap<usize, ResultItem> {
        &self.order_map
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
