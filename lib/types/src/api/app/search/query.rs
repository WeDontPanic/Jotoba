use crate::api::app::{deserialize_lang, deserialize_lang_option};
use crate::jotoba::languages::Language;
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

/// APP settings
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct UserSettings {
    #[serde(deserialize_with = "deserialize_lang")]
    pub user_lang: Language,
    pub show_english: bool,
    pub page_size: u32,
    pub kanji_page_size: u32,
    pub show_example_sentences: bool,
    pub sentence_furigana: bool,
}
