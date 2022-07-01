#[cfg(feature = "jotoba_intern")]
use localization::{language::Language, traits::Translatable, TranslationDict};

use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Hash)]
pub enum SearchTarget {
    #[serde(rename = "1")]
    Kanji,
    #[serde(rename = "2")]
    Sentences,
    #[serde(rename = "3")]
    Names,
    #[serde(rename = "0", other)]
    Words,
}

impl SearchTarget {
    /// Iterate over all query types
    pub fn iterate() -> impl Iterator<Item = Self> {
        vec![Self::Kanji, Self::Sentences, Self::Names, Self::Words].into_iter()
    }

    #[cfg(feature = "jotoba_intern")]
    pub fn get_translated<'a>(
        &self,
        dict: &'a TranslationDict,
        language: Option<Language>,
    ) -> &'a str {
        dict.gettext(self.get_id(), language)
    }

    #[inline]
    pub fn get_type_id(&self) -> u8 {
        match self {
            SearchTarget::Kanji => 1,
            SearchTarget::Sentences => 2,
            SearchTarget::Names => 3,
            SearchTarget::Words => 0,
        }
    }
}

impl TryFrom<u8> for SearchTarget {
    type Error = ();

    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Words,
            1 => Self::Kanji,
            2 => Self::Sentences,
            3 => Self::Names,
            _ => return Err(()),
        })
    }
}

impl Default for SearchTarget {
    #[inline]
    fn default() -> Self {
        Self::Words
    }
}

#[cfg(feature = "jotoba_intern")]
impl Translatable for SearchTarget {
    #[inline]
    fn get_id(&self) -> &'static str {
        match self {
            SearchTarget::Kanji => "Kanji",
            SearchTarget::Sentences => "Sentences",
            SearchTarget::Names => "Names",
            SearchTarget::Words => "Words",
        }
    }
}
