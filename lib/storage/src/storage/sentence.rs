use std::collections::HashMap;

use intmap::IntMap;
use serde::{Deserialize, Serialize};
use types::jotoba::sentences::Sentence;

use super::feature::Feature;

/// Storage for sentence related data
#[derive(Serialize, Deserialize, Default, Clone)]
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

    pub fn get_features(&self) -> Vec<Feature> {
        let mut out = vec![];

        if !self.sentences.is_empty() {
            out.push(Feature::Sentences);
        }

        if !self.jlpt_map.is_empty() {
            out.push(Feature::SentenceJLPT);
        }

        out
    }
}
