#[cfg(feature = "jotoba_intern")]
use localization::{language::Language, traits::Translatable, TranslationDict};

use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Hash)]
pub enum QueryType {
    #[serde(rename = "1")]
    Kanji,
    #[serde(rename = "2")]
    Sentences,
    #[serde(rename = "3")]
    Names,
    #[serde(rename = "0", other)]
    Words,
}

impl QueryType {
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
            QueryType::Kanji => 1,
            QueryType::Sentences => 2,
            QueryType::Names => 3,
            QueryType::Words => 0,
        }
    }
}

impl Default for QueryType {
    #[inline]
    fn default() -> Self {
        Self::Words
    }
}

#[cfg(feature = "jotoba_intern")]
impl Translatable for QueryType {
    #[inline]
    fn get_id(&self) -> &'static str {
        match self {
            QueryType::Kanji => "Kanji",
            QueryType::Sentences => "Sentences",
            QueryType::Names => "Names",
            QueryType::Words => "Words",
        }
    }
}
