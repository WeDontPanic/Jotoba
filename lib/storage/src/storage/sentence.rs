use std::collections::HashMap;

use intmap::IntMap;
use serde::{Deserialize, Serialize};
use types::jotoba::sentences::Sentence;

/// Storage for sentence related data
#[derive(Serialize, Deserialize, Default)]
pub struct SentenceStorage {
    /// Mapping sentence by its ID
    pub sentences: IntMap<Sentence>,

    // Search tags
    pub jlpt_map: HashMap<u8, Vec<u32>>,
}

impl SentenceStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
