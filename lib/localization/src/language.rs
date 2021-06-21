use strum_macros::{AsRefStr, Display, EnumString};

/// Supported languages for translation
#[derive(Copy, Clone, AsRefStr, EnumString, Display, Eq, PartialEq, Debug, Hash)]
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
    fn default() -> Self {
        Self::English
    }
}
