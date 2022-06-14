use ids_parser::IDS;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::jotoba::kanji::{radical::DetailedRadical, Kanji};

use super::feature::Feature;

/// Storage containing all data related to kanji
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KanjiStorage {
    /// Index mapping kanji literals to `Kanji` data
    pub literal_index: intmap::IntMap<Kanji>,

    /// Mapping from a radical to a list of kanji using this radical
    pub radical_map: HashMap<char, Vec<char>>,

    /// Maps radical literal to its detailed radical data
    pub radical_data: HashMap<char, DetailedRadical>,

    /// Jlpt mapping for kanji
    pub jlpt_data: HashMap<u8, Vec<char>>,

    // Search tags
    pub genki_levels: HashMap<u8, Vec<char>>,

    /// IDS index for kanji decomposition graph
    pub ids_index: HashMap<char, IDS>,

    has_similar_kanji: bool,
}

impl KanjiStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert kanji into the KanjiStorage
    pub fn insert_kanji(&mut self, kanji: Vec<Kanji>) {
        self.literal_index.clear();
        self.jlpt_data.clear();

        for kanji in kanji {
            if let Some(jlpt) = kanji.jlpt {
                self.jlpt_data.entry(jlpt).or_default().push(kanji.literal);
            }

            if !self.has_similar_kanji && !kanji.similar_kanji.is_empty() {
                self.has_similar_kanji = true;
            }
            self.literal_index.insert(kanji.literal as u32, kanji);
        }
    }

    /// Insert radical detail data
    pub fn insert_radicals(&mut self, radicals: Vec<DetailedRadical>) {
        self.radical_data.clear();
        for radical in radicals {
            self.radical_data.insert(radical.literal, radical);
        }
    }

    pub fn get_features(&self) -> Vec<Feature> {
        let mut out = vec![];

        if !self.literal_index.is_empty() {
            out.push(Feature::Kanji);
        }

        if !self.genki_levels.is_empty() {
            out.push(Feature::GenkiTags);
        }

        if !self.radical_data.is_empty() {
            out.push(Feature::RadicalData);
        }

        if !self.radical_map.is_empty() {
            out.push(Feature::RadicalKanjiMap);
        }

        if self.has_similar_kanji {
            out.push(Feature::SimilarKanji);
        }

        if !self.ids_index.is_empty() {
            out.push(Feature::KanjiDecompositions);
        }

        out
    }
}
