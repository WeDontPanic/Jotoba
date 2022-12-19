pub mod completions;
pub mod details;
pub mod image;
pub mod kanji;
pub mod news;
pub mod radical;
pub mod search;

use crate::jotoba::language::Language;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/// Deserializes a field into a Option<Language>. None if invalid lang-str, empty or Deserializing str
/// failed
#[inline]
pub fn deserialize_lang_option<'de, D>(s: D) -> Result<Option<Language>, D::Error>
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
#[inline]
pub fn deserialize_lang<'de, D>(s: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let lang = Language::from_str(&String::deserialize(s)?).unwrap_or_default();
    return Ok(lang);
}
