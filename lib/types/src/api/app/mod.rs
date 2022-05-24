use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::jotoba::languages::Language;

pub mod details;
pub mod search;

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
fn deserialize_lang<'de, D>(s: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let lang = Language::from_str(&String::deserialize(s)?).unwrap_or_default();
    return Ok(lang);
}
