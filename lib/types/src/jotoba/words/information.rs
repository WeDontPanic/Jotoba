#[cfg(feature = "jotoba_intern")]
use localization::traits::Translatable;
use strum_macros::{AsRefStr, EnumString};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize, Deserialize, Hash, Eq)]
#[repr(u8)]
pub enum Information {
    #[strum(serialize = "ateji")]
    Ateji,
    #[strum(serialize = "ik")]
    IrregularKana,
    #[strum(serialize = "iK")]
    IrregularKanji,
    #[strum(serialize = "io")]
    IrregularOkurigana,
    #[strum(serialize = "oK")]
    OutdatedKanji,
    #[strum(serialize = "ok")]
    OutdatedKana,
    #[strum(serialize = "gikun")]
    Gikun,
    #[strum(serialize = "uK")]
    UsuallyKana,
    #[strum(serialize = "rK")]
    RarelyUsedKanjiForm,
}

#[cfg(feature = "jotoba_intern")]
impl Translatable for Information {
    fn get_id(&self) -> &'static str {
        match self {
            Information::Ateji => "ateji",
            Information::IrregularKana => "irregular kana",
            Information::IrregularKanji => "irregular kanji",
            Information::IrregularOkurigana => "irregular okurigana",
            Information::OutdatedKanji => "outdated kanji",
            Information::OutdatedKana => "outdated kana",
            Information::Gikun => "gikun",
            Information::UsuallyKana => "usually written in kana",
            Information::RarelyUsedKanjiForm => "rarely used kanji form",
        }
    }
}
