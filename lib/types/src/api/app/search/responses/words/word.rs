use serde::Serialize;

use crate::jotoba::{
    languages::Language,
    words::{
        dialect::Dialect, field::Field, misc::Misc, part_of_speech::PartOfSpeech, sense::Gairaigo,
    },
};

/// A single word item
#[derive(Clone, Serialize)]
pub struct Word {
    pub sequence: u32,
    pub is_common: bool,
    pub reading: String,
    pub alt_readings: Vec<String>,
    pub senses: Vec<Sense>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accents: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furigana: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jlpt_lvl: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transive_verion: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intransive_verion: Option<u32>,
    pub sentences_available: u16,
}

#[derive(Clone, Serialize)]
pub struct Sense {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc: Option<Misc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialect: Option<Dialect>,
    pub glosses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antonym: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<String>,
    pub part_of_speech: Vec<PartOfSpeech>,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_sentence: Option<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gairaigo: Option<Gairaigo>,
}
