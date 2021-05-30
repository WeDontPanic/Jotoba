use strum_macros::{AsRefStr, Display, EnumString};

/// Supported languages for translation
#[derive(Copy, Clone, AsRefStr, EnumString, Display, Eq, PartialEq, Debug, Hash)]
pub enum Language {
    #[strum(serialize = "en")]
    English,
    #[strum(serialize = "de")]
    German,
    #[strum(serialize = "ru")]
    Russain,
    #[strum(serialize = "sp")]
    Spanish,
    #[strum(serialize = "sw")]
    Swedish,
    #[strum(serialize = "fr")]
    French,
    #[strum(serialize = "nl")]
    Dutch,
    #[strum(serialize = "hu")]
    Hungarian,
    #[strum(serialize = "sv")]
    Slovenian,
    #[strum(serialize = "jp")]
    Japanese,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}
