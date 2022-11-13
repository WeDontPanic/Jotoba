use serde::{Deserialize, Serialize};

use crate::jotoba::kanji::radical::DetailedRadical;

/// Kanji API response. Contains all kanji
#[derive(Clone, Debug, Serialize)]
pub struct KanjiResponse {
    kanji: Vec<Kanji>,
}

impl KanjiResponse {
    #[inline]
    pub fn new(kanji: Vec<Kanji>) -> Self {
        Self { kanji }
    }
}

/// Kanji information
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub nanori: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub similar_kanji: Vec<char>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parts: Vec<char>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vietnamese: Vec<String>,
    pub has_compounds: bool,
    pub radical: DetailedRadical,
}

impl From<crate::jotoba::kanji::Kanji> for Kanji {
    #[inline]
    fn from(k: crate::jotoba::kanji::Kanji) -> Self {
        let has_compounds = !k.on_dicts.is_empty() || !k.kun_dicts.is_empty();
        Self {
            literal: k.literal,
            stroke_count: k.stroke_count,
            grade: k.grade,
            frequency: k.frequency,
            jlpt: k.jlpt,
            onyomi: k.onyomi,
            kunyomi: k.kunyomi,
            variant: k.variant,
            chinese: k.chinese,
            korean_romaji: k.korean_r,
            korean_hangul: k.korean_h,
            nanori: k.nanori,
            similar_kanji: k.similar_kanji,
            meanings: k.meanings,
            parts: k.parts,
            radical: k.radical,
            vietnamese: k.vietnamese,
            has_compounds,
        }
    }
}
