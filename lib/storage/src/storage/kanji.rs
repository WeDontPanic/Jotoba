use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::jotoba::kanji::Kanji;

/// Storage containing all data related to kanji
#[derive(Serialize, Deserialize, Default)]
pub struct KanjiStorage {
    /// Index mapping kanji literals to `Kanji` data
    literal_index: HashMap<char, Kanji>,

    /// Mapping from a radical to a list of kanji using this radical
    radical_map: HashMap<char, Vec<char>>,

    // Search tags
    genki_levels: HashMap<u8, Vec<char>>,
    jlpt_data: HashMap<u8, Vec<char>>,
}

impl KanjiStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
