use super::feature::Feature;
use intmap::IntMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::jotoba::sentences::{tag::Tag, Sentence};

/// Storage for sentence related data
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SentenceStorage {
    /// Mapping sentence by its ID
    pub sentences: IntMap<Sentence>,

    /// Mappings of tags to sentences with this tag
    pub tag_map: HashMap<Tag, Vec<u32>>,

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

        if !self.tag_map.is_empty() {
            out.push(Feature::SentenceTags);
        }

        if !self.jlpt_map.is_empty() {
            out.push(Feature::SentenceJLPT);
        }

        out
    }
}
