use serde::{Deserialize, Serialize};

use crate::jotoba::languages::Language;

#[derive(Serialize, Deserialize)]
pub struct Response {
    sentences: Vec<Sentence>,
}

#[derive(Serialize, Deserialize)]
pub struct Sentence {
    pub content: String,
    pub furigana: String,
    pub translation: String,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eng: Option<String>,
}

impl From<Vec<Sentence>> for Response {
    #[inline]
    fn from(sentences: Vec<Sentence>) -> Self {
        Self { sentences }
    }
}
