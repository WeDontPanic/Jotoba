use serde::{Deserialize, Serialize};
use std::collections::{hash_map::Iter, HashMap, HashSet};

/// Index to allow fast and efficient regex search queries.
#[derive(Serialize, Deserialize)]
pub struct RegexSearchIndex {
    data: HashMap<char, HashSet<u32>>,
}

impl RegexSearchIndex {
    /// Creates a new empty Index
    #[inline]
    pub fn new() -> Self {
        RegexSearchIndex {
            data: HashMap::new(),
        }
    }

    /// Returns an iterator over all items in the index
    #[inline]
    pub fn iter(&self) -> Iter<char, HashSet<u32>> {
        self.data.iter()
    }

    /// Returns a HashSet with all words (seq_ids) that contain the given character
    #[inline(always)]
    pub fn get_words_with(&self, character: char) -> Option<&HashSet<u32>> {
        self.data.get(&character)
    }

    /// Adds a new term to the index
    #[inline]
    pub fn add_term(&mut self, term: &str, seq_id: u32) {
        for c in term.chars() {
            self.data.entry(c).or_default().insert(seq_id);
        }
    }
}
