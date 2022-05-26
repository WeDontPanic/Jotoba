use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::jotoba::kanji::{DetailedRadical, Kanji};

/// Storage containing all data related to kanji
#[derive(Serialize, Deserialize, Default)]
pub struct KanjiStorage {
    /// Index mapping kanji literals to `Kanji` data
    pub literal_index: HashMap<char, Kanji>,

    /// Mapping from a radical to a list of kanji using this radical
    pub radical_map: HashMap<char, Vec<char>>,

    /// Maps radical literal to its detailed radical data
    pub radical_data: HashMap<char, DetailedRadical>,

    /// Jlpt mapping for kanji
    pub jlpt_data: HashMap<u8, Vec<char>>,

    // Search tags
    pub genki_levels: HashMap<u8, Vec<char>>,
}

impl KanjiStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
