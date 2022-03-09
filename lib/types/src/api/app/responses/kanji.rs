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
    pub grade: Option<u8>,
    pub frequency: Option<u16>,
    pub jlpt: Option<u8>,
    pub onyomi: Vec<String>,
    pub kunyomi: Vec<String>,
    pub variant: Vec<String>,
    pub chinese: Vec<String>,
    pub korean_romaji: Vec<String>,
    pub korean_hangul: Vec<String>,
    pub natori: Vec<String>,
    pub similar_kanji: Vec<char>,
    pub meanings: Vec<String>,
    pub parts: Vec<char>,
    pub kun_compounds: Vec<CompoundWord>,
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
    pub translations: Vec<String>,
}

impl CompoundWord {
    /// Create a new CompoundWord
    pub fn new(jp: String, translations: Vec<String>) -> Self {
        Self { jp, translations }
    }

    /// Convertes a Word to a CompoundWord. Takes ALL senses and ALL glosses. If you only want
    /// some of the glosses, filter them first
    pub fn from_word(word: &crate::jotoba::words::Word) -> Self {
        let jp = word.get_reading().reading.clone();
        let translations = word
            .senses
            .iter()
            .map(|i| i.glosses.clone())
            .flatten()
            .map(|i| i.gloss)
            .collect::<Vec<String>>();
        Self { jp, translations }
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
