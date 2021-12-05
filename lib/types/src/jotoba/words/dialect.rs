use std::fmt::Display;

#[cfg(feature = "jotoba_intern")]
use localization::{language::Language, traits::Translatable, TranslationDict};

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Clone, Copy, EnumString, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum Dialect {
    #[strum(serialize = "bra")]
    Brazilian,
    #[strum(serialize = "hob")]
    Hokkaido,
    #[strum(serialize = "ksb")]
    Kansai,
    #[strum(serialize = "ktb")]
    Kantou,
    #[strum(serialize = "kyb")]
    Kyoto,
    #[strum(serialize = "kyu")]
    Kyuushuu,
    #[strum(serialize = "nab")]
    Nagano,
    #[strum(serialize = "osb")]
    Osaka,
    #[strum(serialize = "rkb")]
    Ryuukyuu,
    #[strum(serialize = "thb")]
    Touhoku,
    #[strum(serialize = "tsb")]
    Tosa,
    #[strum(serialize = "tsug")]
    Tsugaru,
}

impl Display for Dialect {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<&'static str> for Dialect {
    #[inline]
    fn into(self) -> &'static str {
        match self {
            Dialect::Hokkaido => "Hokkaido",
            Dialect::Brazilian => "Brazilian",
            Dialect::Kansai => "Kansai",
            Dialect::Kantou => "Kantou",
            Dialect::Kyoto => "Kyoto",
            Dialect::Kyuushuu => "Kyuushuu",
            Dialect::Nagano => "Nagano",
            Dialect::Osaka => "Osaka",
            Dialect::Ryuukyuu => "Ryuukyuu",
            Dialect::Touhoku => "Touhoku",
            Dialect::Tosa => "Tosa",
            Dialect::Tsugaru => "Tsugaru",
        }
    }
}

#[cfg(feature = "jotoba_intern")]
impl Translatable for Dialect {
    #[inline]
    fn get_id(&self) -> &'static str {
        (*self).into()
    }

    #[inline]
    fn gettext_custom(&self, dict: &TranslationDict, language: Option<Language>) -> String {
        dict.gettext_fmt("{} dialect", &[self.gettext(dict, language)], language)
    }
}
