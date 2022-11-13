use serde::{Deserialize, Serialize};

/// Response for kanji compound request
#[derive(Deserialize, Serialize)]
pub struct CompoundResponse {
    pub compounds: Vec<CompoundSet>,
}

impl CompoundResponse {
    #[inline]
    pub fn new(compounds: Vec<CompoundSet>) -> Self {
        Self { compounds }
    }
}

/// Set of compounds for a single kanji
#[derive(Deserialize, Serialize)]
pub struct CompoundSet {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub on: Vec<CompoundWord>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub kun: Vec<CompoundWord>,
}

impl CompoundSet {
    #[inline]
    pub fn new(on: Vec<CompoundWord>, kun: Vec<CompoundWord>) -> Self {
        Self { on, kun }
    }
}

/// A word used in kanji compounds
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompoundWord {
    pub jp: String,
    pub kana: String,
    pub translations: Vec<String>,
}

impl CompoundWord {
    /// Create a new CompoundWord
    pub fn new(jp: String, kana: String, translations: Vec<String>) -> Self {
        Self {
            jp,
            kana,
            translations,
        }
    }

    /// Convertes a Word to a CompoundWord. Takes ALL senses and ALL glosses. If you only want
    /// some of the glosses, filter them first
    pub fn from_word(word: &crate::jotoba::words::Word) -> Self {
        let jp = word.get_reading().reading.clone();
        let kana = word.reading.kana.reading.clone();
        let translations = word
            .senses
            .iter()
            .map(|i| i.glosses.clone())
            .flatten()
            .map(|i| i.gloss)
            .collect::<Vec<String>>();
        Self::new(jp, kana, translations)
    }
}
