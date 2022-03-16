use serde::Serialize;

use crate::jotoba::kanji::DetailedRadical;

/// Kanji API response. Contains all kanji
#[derive(Clone, Debug, Serialize)]
pub struct Response {
    kanji: Vec<Kanji>,
}

impl Response {
    pub fn new(kanji: Vec<Kanji>) -> Self {
        Self { kanji }
    }
}

/// Kanji information
#[derive(Clone, Debug, Serialize)]
pub struct Kanji {
    pub literal: char,
    pub stroke_count: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jlpt: Option<u8>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub onyomi: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub kunyomi: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variant: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chinese: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub korean_romaji: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub korean_hangul: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub natori: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub similar_kanji: Vec<char>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<char>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub kun_compounds: Vec<CompoundWord>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub on_compounds: Vec<CompoundWord>,
    pub radical: DetailedRadical,
}

impl Kanji {
    /// Set the kanji's kun compounds.
    pub fn set_kun_compounds(&mut self, kun_compounds: Vec<CompoundWord>) {
        self.kun_compounds = kun_compounds;
    }

    /// Set the kanji's on compounds.
    pub fn set_on_compounds(&mut self, on_compounds: Vec<CompoundWord>) {
        self.on_compounds = on_compounds;
    }
}

/// A word used in kanji compounds
#[derive(Clone, Debug, Serialize)]
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

impl From<crate::jotoba::kanji::Kanji> for Kanji {
    #[inline]
    fn from(k: crate::jotoba::kanji::Kanji) -> Self {
        Self {
            literal: k.literal,
            stroke_count: k.stroke_count,
            grade: k.grade,
            frequency: k.frequency,
            jlpt: k.jlpt,
            onyomi: k.onyomi.unwrap_or_default(),
            kunyomi: k.kunyomi.unwrap_or_default(),
            variant: k.variant.unwrap_or_default(),
            chinese: k.chinese.unwrap_or_default(),
            korean_romaji: k.korean_r.unwrap_or_default(),
            korean_hangul: k.korean_h.unwrap_or_default(),
            natori: k.natori.unwrap_or_default(),
            similar_kanji: k.similar_kanji.unwrap_or_default(),
            meanings: k.meanings,
            parts: k.parts.unwrap_or_default(),
            radical: k.radical,
            kun_compounds: vec![],
            on_compounds: vec![],
        }
    }
}
