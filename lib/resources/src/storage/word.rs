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

    // Feature information
    has_accents: bool,
    has_sentence_mapping: bool,
    has_jlpt: bool,
}

impl WordStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the amounot of words in the WordStorage
    #[inline]
    pub fn count(&self) -> usize {
        self.words.len()
    }

    /// Inserts words into the WordStorage
    pub fn insert_words(&mut self, words: Vec<Word>) {
        self.clear();

        for word in words {
            if let Some(jlpt) = word.jlpt_lvl {
                self.jlpt_word_map
                    .entry(jlpt)
                    .or_default()
                    .push(word.sequence);
                self.has_jlpt = true;
            }

            if !self.has_accents && word.accents.count() > 0 {
                self.has_accents = true;
            }

            self.words.insert(word.sequence, word);
        }

        for (_, v) in self.jlpt_word_map.iter_mut() {
            v.sort();
        }
    }

    pub fn update_sentence_mapping(&mut self) {
        self.has_sentence_mapping = self.words.iter().any(|i| i.1.sentences_available > 0);
    }

    pub fn get_features(&self) -> Vec<Feature> {
        let mut out = vec![];

        if !self.words.is_empty() {
            out.push(Feature::Words);
        }

        if !self.irregular_ichidan.is_empty() {
            out.push(Feature::WordIrregularIchidan);
        }

        if self.has_sentence_mapping {
            out.push(Feature::SentenceAvailable);
        }

        if self.has_accents {
            out.push(Feature::WordPitch);
        }

        if self.has_jlpt {
            out.push(Feature::WordJlpt);
        }

        out
    }

    fn clear(&mut self) {
        self.words.clear();
        self.jlpt_word_map.clear();
        self.has_accents = false;
        self.has_sentence_mapping = false;
    }
}
