use strum_macros::{AsRefStr, Display, EnumString};

use crate::traits::Translatable;

/// Supported languages for translation
#[derive(Copy, Clone, AsRefStr, EnumString, Display, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum Language {
    #[strum(serialize = "en", serialize = "en-US")]
    English,
    #[strum(serialize = "de", serialize = "de-DE")]
    German,
    #[strum(serialize = "ru")]
    Russain,
    #[strum(serialize = "sp", serialize = "es-ES")]
    Spanish,
    #[strum(serialize = "sw", serialize = "sv-SE")]
    Swedish,
    #[strum(serialize = "fr", serialize = "fr-FR")]
    French,
    #[strum(serialize = "nl", serialize = "nl-NL")]
    Dutch,
    #[strum(serialize = "hu")]
    Hungarian,
    #[strum(serialize = "sv", serialize = "sl-SL", serialize = "svl")]
    Slovenian,
    #[strum(serialize = "jp", serialize = "ja-JP")]
    Japanese,
}

impl Default for Language {
    #[inline]
    fn default() -> Self {
        Self::English
    }
}

impl Translatable for Language {
    #[inline]
    fn get_id(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::German => "German",
            Language::Russain => "Russian",
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
