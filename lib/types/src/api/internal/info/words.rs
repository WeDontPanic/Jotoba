use crate::{
    api::app::deserialize_lang,
    jotoba::{
        language::{LangParam, Language},
        sentences::Sentence,
        words::{part_of_speech::PosSimple, Word},
    },
};
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
    pub items: Vec<WordItem>,
}

#[derive(Serialize, Deserialize)]
pub struct WordItem {
    pub word: Word,
    pub sentences: Vec<Sentence>,
    pub audio: Option<String>,
    pub pos: Vec<PosSimple>,
}

impl WordItem {
    pub fn new(
        word: Word,
        sentences: Vec<Sentence>,
        audio: Option<String>,
        pos: Vec<PosSimple>,
    ) -> Self {
        Self {
            word,
            sentences,
            audio,
            pos,
        }
    }
}

impl Response {
    #[inline]
    pub fn new(items: Vec<WordItem>) -> Self {
        Self { items }
    }
}
