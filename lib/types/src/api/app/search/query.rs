use crate::{
    api::app::{deserialize_lang, deserialize_lang_option},
    jotoba::language::{LangParam, Language},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchPayload {
    pub settings: UserSettings,

    /// Searched query text
    pub query_str: String,

    /// Result page
    #[serde(default)]
    pub page: Option<u32>,

    /// Index in sentence reader
    #[serde(default)]
    pub word_index: Option<usize>,

    /// Overwrite
    #[serde(default, deserialize_with = "deserialize_lang_option")]
    pub lang_overwrite: Option<Language>,
}

impl SearchPayload {
    /// Returns language parameters for the query
    #[inline]
    pub fn lang_param(&self) -> LangParam {
        self.settings.lang_param()
    }
}

/// APP settings
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct UserSettings {
    #[serde(deserialize_with = "deserialize_lang")]
    pub user_lang: Language,
    pub show_english: bool,
    pub page_size: u32,
    pub show_example_sentences: bool,
    pub sentence_furigana: bool,
}

impl UserSettings {
    /// Returns language parameters for user settinsg
    #[inline]
    pub fn lang_param(&self) -> LangParam {
        LangParam::with_en_raw(self.user_lang, self.show_english)
    }
}
