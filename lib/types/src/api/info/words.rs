use crate::jotoba::languages::Language;
use crate::{api::app::deserialize_lang, jotoba::words::Word};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub ids: Vec<u32>,
    #[serde(deserialize_with = "deserialize_lang")]
    pub language: Language,
    pub show_english: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub words: Vec<Word>,
}

impl Response {
    pub fn new(words: Vec<Word>) -> Self {
        Self { words }
    }
}
