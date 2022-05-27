use intmap::IntMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::jotoba::words::Word;

use super::feature::Feature;

/// Storage containing all data related to words
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct WordStorage {
    /// Word index
    pub words: IntMap<Word>,

    // Search tags
    pub jlpt_word_map: HashMap<u8, Vec<u32>>,
    pub irregular_ichidan: Vec<u32>,
}

impl WordStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the amounot of words in the WordStorage
    #[inline]
    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    /// Inserts words into the WordStorage
    pub fn insert_words(&mut self, words: Vec<Word>) {
        self.words.clear();
        self.jlpt_word_map.clear();

        for word in words {
            if let Some(jlpt) = word.jlpt_lvl {
                self.jlpt_word_map
                    .entry(jlpt)
                    .or_default()
                    .push(word.sequence);
            }

            self.words.insert(word.sequence as u64, word);
        }

        for (_, v) in self.jlpt_word_map.iter_mut() {
            v.sort();
        }
    }

    pub fn get_features(&self) -> Vec<Feature> {
        let mut out = vec![];

        if !self.words.is_empty() {
            out.push(Feature::Words);
        }

        if !self.irregular_ichidan.is_empty() {
            out.push(Feature::WordIrregularIchidan);
        }

        if self.words.iter().any(|i| i.1.accents.count() > 0) {
            out.push(Feature::WordPitch);
        }

        out
    }
}
