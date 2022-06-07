use std::collections::HashMap;

use bktree::BkTree;
use serde::{Deserialize, Serialize};
use types::jotoba::kanji::radical::SearchRadicalInfo;

/// Radicals indexed by its meanings
#[derive(Serialize, Deserialize)]
pub struct RadicalIndex {
    pub meaning_map: HashMap<String, Vec<SearchRadicalInfo>>,
    pub term_tree: BkTree<String>,
}

impl RadicalIndex {
    /// Returns `true` if the index contains `term`
    #[inline(always)]
    pub fn has_term(&self, term: &str) -> bool {
        self.meaning_map.contains_key(term)
    }

    /// Returns `SearchRadicalInfo` from the index by its term or `None` if term is not found
    #[inline(always)]
    pub fn get(&self, term: &str) -> Option<&Vec<SearchRadicalInfo>> {
        self.meaning_map.get(term)
    }
}
