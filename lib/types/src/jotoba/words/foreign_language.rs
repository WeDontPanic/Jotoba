#[cfg(feature = "jotoba_intern")]
use localization::traits::Translatable;

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum ForeignLanguage {
    #[strum(serialize = "eng")]
    English,
    #[strum(serialize = "geo")]
    Georgian,
    #[strum(serialize = "ger")]
    German,
    #[strum(serialize = "chi")]
    Chinese,
    #[strum(serialize = "may")]
    Manchu,
    #[strum(serialize = "kur")]
    Kurdish,
    #[strum(serialize = "mnc")]
    ChinookJargon,
    #[strum(serialize = "ita")]
    Italian,
    #[strum(serialize = "mal")]
    Malayalam,
    #[strum(serialize = "tib")]
    Tibetian,
    #[strum(serialize = "m")]
    Mongolian,
    #[strum(serialize = "ru")]
    Romanian,
    #[strum(serialize = "b")]
    Bantu,
    #[strum(serialize = "nor")]
    Norwegian,
    #[strum(serialize = "gr", serialize = "grc")]
    Greek,
    #[strum(serialize = "ice")]
    Icelandic,
    #[strum(serialize = "br")]
    Breton,
    #[strum(serialize = "mao")]
    Maori,
    #[strum(serialize = "lat")]
    Latin,
    #[strum(serialize = "amh")]
    Amharic,
    #[strum(serialize = "khm")]
    Khmer,
    #[strum(serialize = "swa")]
    Swahili,
    #[strum(serialize = "heb")]
    Hebrew,
    #[strum(serialize = "glg")]
    Galician,
    #[strum(serialize = "kor")]
    Korean,
    #[strum(serialize = "tam")]
    Tamil,
    #[strum(serialize = "vie")]
    Viatnamese,
    #[strum(serialize = "pol")]
    Polish,
    #[strum(serialize = "san")]
    Sanskrit,
    #[strum(serialize = "per")]
    Persian,
    #[strum(serialize = "fil")]
    Filipino,
    #[strum(serialize = "mol")]
    Moldavian,
    #[strum(serialize = "scr")]
    Croatian,
    #[strum(serialize = "tha")]
    Thai,
    #[strum(serialize = "bur")]
    Burmese,
    #[strum(serialize = "slo")]
    Slovak,
    #[strum(serialize = "cze")]
    Czech,
    #[strum(serialize = "hin")]
    Hindi,
    #[strum(serialize = "arn")]
    Mapudungun,
    #[strum(serialize = "tur")]
    Turkish,
    #[strum(serialize = "haw")]
    Hawaiian,
    #[strum(serialize = "afr")]
    Afrikaans,
    #[strum(serialize = "epo")]
    Esperanto,
    #[strum(serialize = "yid")]
    Yiddish,
    #[strum(serialize = "som")]
    Somali,
    #[strum(serialize = "tah")]
    Tahitian,
    #[strum(serialize = "urd")]
    Urdu,
    #[strum(serialize = "ind")]
    Indonesian,
    #[strum(serialize = "est")]
    Estonian,
    #[strum(serialize = "bul")]
    Bulgarian,
    #[strum(serialize = "ara")]
    Arabic,
    #[strum(serialize = "dan")]
    Danish,
    #[strum(serialize = "por")]
    Portuguese,
    #[strum(serialize = "fin")]
    Finnish,
    #[strum(serialize = "ain")]
    Ainu,
    #[strum(serialize = "alg")]
    Algonquian,
    #[strum(serialize = "fre")]
    French,
}

#[cfg(feature = "jotoba_intern")]
impl Translatable for ForeignLanguage {
    fn get_id(&self) -> &'static str {
        match self {
            ForeignLanguage::English => "English",
            ForeignLanguage::Georgian => "Georgian",
            ForeignLanguage::German => "German",
            ForeignLanguage::Chinese => "Chinese",
            ForeignLanguage::Manchu => "Manchu",
            ForeignLanguage::Kurdish => "Kurdish",
            ForeignLanguage::ChinookJargon => "ChinookJargon",
            ForeignLanguage::Italian => "Italian",
            ForeignLanguage::Malayalam => "Malayalam",
            ForeignLanguage::Tibetian => "Tibetian",
            ForeignLanguage::Mongolian => "Mongolian",
            ForeignLanguage::Romanian => "Romanian",
            ForeignLanguage::Bantu => "Bantu",
            ForeignLanguage::Norwegian => "Norwegian",
            ForeignLanguage::Greek => "Greek",
            ForeignLanguage::Icelandic => "Icelandic",
            ForeignLanguage::Breton => "Breton",
            ForeignLanguage::Maori => "Maori",
            ForeignLanguage::Latin => "Latin",
            ForeignLanguage::Amharic => "Amharic",
            ForeignLanguage::Khmer => "Khmer",
            ForeignLanguage::Swahili => "Swahili ",
            ForeignLanguage::Hebrew => "Hebrew",
            ForeignLanguage::Galician => "Galician",
            ForeignLanguage::Korean => "Korean",
            ForeignLanguage::Tamil => "Tamil",
            ForeignLanguage::Viatnamese => "Viatnamese",
            ForeignLanguage::Polish => "Polish",
            ForeignLanguage::Sanskrit => "Sanskrit",
            ForeignLanguage::Persian => "Persian",
            ForeignLanguage::Filipino => "Filipino",
            ForeignLanguage::Moldavian => "Moldavian",
            ForeignLanguage::Croatian => "Croatian",
            ForeignLanguage::Thai => "Thai",
            ForeignLanguage::Burmese => "Burmese",
            ForeignLanguage::Slovak => "Slovak",
            ForeignLanguage::Czech => "Czech",
            ForeignLanguage::Hindi => "Hindi",
            ForeignLanguage::Mapudungun => "Mapudungun",
            ForeignLanguage::Turkish => "Turkish",
            ForeignLanguage::Hawaiian => "Hawaiian",
            ForeignLanguage::Afrikaans => "Afrikaans",
            ForeignLanguage::Esperanto => "Esperanto",
            ForeignLanguage::Yiddish => "Yiddish",
            ForeignLanguage::Somali => "Somali",
            ForeignLanguage::Tahitian => "Tahitian",
            ForeignLanguage::Urdu => "Urdu",
            ForeignLanguage::Indonesian => "Indonesian",
            ForeignLanguage::Estonian => "Estonian",
            ForeignLanguage::Bulgarian => "Bulgarian",
            ForeignLanguage::Arabic => "Arabic",
            ForeignLanguage::Danish => "Danish",
            ForeignLanguage::Portuguese => "Portuguese",
            ForeignLanguage::Finnish => "Finnish",
            ForeignLanguage::Ainu => "Ainu",
            ForeignLanguage::Algonquian => "Algonquian",
            ForeignLanguage::French => "French",
        }
    }
}

impl Default for ForeignLanguage {
    #[inline]
    fn default() -> Self {
        Self::English
    }
}
