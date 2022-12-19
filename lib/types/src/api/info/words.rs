use crate::jotoba::language::{LangParam, Language};
use crate::{api::app::deserialize_lang, jotoba::words::Word};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub ids: Vec<u32>,
    #[serde(deserialize_with = "deserialize_lang")]
    pub language: Language,
    pub show_english: bool,
}

impl Request {
    #[inline]
    pub fn new(ids: Vec<u32>, language: Language, show_english: bool) -> Self {
        Self {
            ids,
            language,
            show_english,
        }
    }

    #[inline]
    pub fn lang_param(&self) -> LangParam {
        LangParam::with_en_raw(self.language, self.show_english)
    }
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
