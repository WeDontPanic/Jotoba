use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::jotoba::languages::Language;

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
    pub english_on_top: bool,
    pub page_size: u32,
    pub kanji_page_size: u32,
    pub show_example_sentences: bool,
    pub sentence_furigana: bool,
}

/// Deserializes a field into a Option<Language>. None if invalid lang-str, empty or Deserializing str
/// failed
fn deserialize_lang_option<'de, D>(s: D) -> Result<Option<Language>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(s)?;
    if s.trim().is_empty() {
        return Ok(None);
    }
    return Ok(Language::from_str(&s).ok());
}

/// Deserializes a field into a Option<Language>. None if invalid lang-str, empty or Deserializing str
/// failed
fn deserialize_lang<'de, D>(s: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let lang = Language::from_str(&String::deserialize(s)?).unwrap_or_default();
    return Ok(lang);
}
