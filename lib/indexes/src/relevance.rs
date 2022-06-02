use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use types::jotoba::languages::Language;
use vector_space_model2::Vector;

#[derive(Serialize, Deserialize)]
pub struct RelevanceIndex {
    pub language: Language,
    pub inner: HashMap<(u32, u16), Vector>,
    pub important_terms: HashSet<u32>,
    pub frequency_map: HashMap<u32, f32>,
}

impl RelevanceIndex {
    pub fn new(
        language: Language,
        inner: HashMap<(u32, u16), Vector>,
        important_terms: HashSet<u32>,
        frequency_map: HashMap<u32, f32>,
    ) -> Self {
        Self {
            language,
            inner,
            important_terms,
            frequency_map,
        }
    }

    #[inline]
    pub fn get(&self, seq_id: u32, sense_id: u16) -> Option<&Vector> {
        self.inner.get(&(seq_id, sense_id))
    }

    #[inline]
    pub fn is_important(&self, term_id: u32) -> bool {
        self.important_terms.get(&term_id).is_some()
    }
}
