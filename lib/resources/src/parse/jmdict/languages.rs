use localization::traits::Translatable;
use std::convert::TryFrom;
use strum_macros::{AsRefStr, Display, EnumString};

use crate::parse::error;

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Display, PartialEq, Eq, Clone, Copy, AsRefStr, EnumString, Hash, Deserialize, Serialize,
)]
#[repr(u8)]
pub enum Language {
    #[strum(serialize = "eng", serialize = "en-US")]
    English,
    #[strum(serialize = "ger", serialize = "de-DE", serialize = "deu")]
    German,
    #[strum(serialize = "rus", serialize = "ru")]
    Russian,
    #[strum(serialize = "spa", serialize = "es-ES")]
    Spanish,
    #[strum(serialize = "swe", serialize = "sv-SE")]
    Swedish,
    #[strum(serialize = "fre", serialize = "fr-FR", serialize = "fra")]
    French,
    #[strum(serialize = "dut", serialize = "nl-NL", serialize = "nld")]
    Dutch,
    #[strum(serialize = "hun", serialize = "hu")]
    Hungarian,
    #[strum(serialize = "slv", serialize = "sl-SL", serialize = "svl")]
    Slovenian,
    #[strum(serialize = "jpn", serialize = "ja", serialize = "jp")]
    Japanese,
}

impl Language {
    /// Returns an iterator over all Languages which have words with this language
    pub fn word_iter() -> impl Iterator<Item = Self> {
        [
            Language::English,
            Language::German,
            Language::Russian,
            Language::Spanish,
            Language::Swedish,
            Language::French,
            Language::Dutch,
            Language::Hungarian,
            Language::Slovenian,
        ]
        .into_iter()
    }

    pub fn to_query_format(&self) -> &'static str {
        match *self {
            Language::English => "eng",
            Language::German => "ger",
            Language::Russian => "rus",
            Language::Spanish => "spa",
            Language::Swedish => "swe",
            Language::French => "fre",
            Language::Dutch => "dut",
            Language::Hungarian => "hun",
            Language::Slovenian => "slv",
            Language::Japanese => "jpn",
        }
    }
}

impl Default for Language {
    #[inline]
    fn default() -> Self {
        Self::English
    }
}

impl TryFrom<i32> for Language {
    type Error = error::Error;
    #[inline]
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        Ok(match i {
            0 => Self::English,
            1 => Self::German,
            2 => Self::Russian,
            3 => Self::Spanish,
            4 => Self::Swedish,
            5 => Self::French,
            6 => Self::Dutch,
            7 => Self::Hungarian,
            8 => Self::Slovenian,
            9 => Self::Japanese,
            _ => return Err(error::Error::ParseError),
        })
    }
}

impl Into<i32> for Language {
    #[inline]
    fn into(self) -> i32 {
        match self {
            Self::English => 0,
            Self::German => 1,
            Self::Russian => 2,
            Self::Spanish => 3,
            Self::Swedish => 4,
            Self::French => 5,
            Self::Dutch => 6,
            Self::Hungarian => 7,
            Self::Slovenian => 8,
            Self::Japanese => 9,
        }
    }
}

impl Translatable for Language {
    #[inline]
    fn get_id(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::German => "German",
            Language::Russian => "Russian",
            Language::Spanish => "Spanish",
            Language::Swedish => "Swedish",
            Language::French => "French",
            Language::Dutch => "Dutch",
            Language::Hungarian => "Hungarian",
            Language::Slovenian => "Slovenian",
            Language::Japanese => "Japanese",
        }
    }
}
