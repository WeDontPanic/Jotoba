use serde::Deserialize;

use crate::jotoba::languages::Language;

#[derive(Debug, Clone, Deserialize)]
pub struct SearchPayload {
    pub settings: UserSettings,

    /// Searched query text
    pub query_str: String,

    /// Result page
    pub page: Option<usize>,

    /// Index in sentence reader
    pub word_index: Option<usize>,

    /// Overwrite
    pub lang_overwrite: Option<Language>,
}

/// APP settings
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct UserSettings {
    pub user_lang: Language,
    pub show_english: bool,
    pub english_on_top: bool,
    pub page_size: u32,
    pub kanji_page_size: u32,
    pub show_example_sentences: bool,
    pub sentence_furigana: bool,
}
