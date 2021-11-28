#![allow(clippy::from_over_into)]
use std::convert::TryFrom;

use crate::parse::error;
use localization::{language::Language, traits::Translatable, TranslationDict};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(
    Debug, PartialEq, Clone, Copy, Hash, EnumString, Serialize, Deserialize, Ord, PartialOrd, Eq,
)]
#[repr(u8)]
pub enum PosSimple {
    #[strum(serialize = "adverb", serialize = "adv")]
    Adverb,
    #[strum(serialize = "auxilary", serialize = "aux")]
    Auxilary,
    #[strum(serialize = "conjungation", serialize = "conj")]
    Conjungation,
    #[strum(serialize = "noun", serialize = "n")]
    Noun,
    #[strum(serialize = "prefix", serialize = "pre")]
    Prefix,
    #[strum(serialize = "suffix", serialize = "suf")]
    Suffix,
    #[strum(serialize = "particle", serialize = "part")]
    Particle,
    #[strum(serialize = "sfx")]
    Sfx,
    #[strum(serialize = "verb", serialize = "v")]
    Verb,
    #[strum(serialize = "adjective", serialize = "adj")]
    Adjective,
    #[strum(serialize = "counter", serialize = "count")]
    Counter,
    #[strum(serialize = "expression", serialize = "expr")]
    Expr,
    #[strum(serialize = "interjection", serialize = "inter")]
    Interjection,
    #[strum(serialize = "pronoun", serialize = "pron")]
    Pronoun,
    #[strum(serialize = "numeric", serialize = "nr")]
    Numeric,
    #[strum(serialize = "transitive", serialize = "tr")]
    Transitive,
    #[strum(serialize = "intransitive", serialize = "itr")]
    Intransitive,
    #[strum(serialize = "unclassified", serialize = "unc")]
    Unclassified,
}

impl TryFrom<i32> for PosSimple {
    type Error = error::Error;
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        Ok(match i {
            0 => Self::Adverb,
            1 => Self::Auxilary,
            2 => Self::Conjungation,
            3 => Self::Noun,
            4 => Self::Prefix,
            5 => Self::Suffix,
            6 => Self::Particle,
            7 => Self::Sfx,
            8 => Self::Verb,
            9 => Self::Adjective,
            10 => Self::Counter,
            11 => Self::Expr,
            12 => Self::Interjection,
            13 => Self::Pronoun,
            15 => Self::Numeric,
            16 => Self::Unclassified,
            17 => Self::Intransitive,
            18 => Self::Transitive,
            _ => return Err(error::Error::ParseError),
        })
    }
}

impl Into<i32> for PosSimple {
    fn into(self) -> i32 {
        match self {
            Self::Adverb => 0,
            Self::Auxilary => 1,
            Self::Conjungation => 2,
            Self::Noun => 3,
            Self::Prefix => 4,
            Self::Suffix => 5,
            Self::Particle => 6,
            Self::Sfx => 7,
            Self::Verb => 8,
            Self::Adjective => 9,
            Self::Counter => 10,
            Self::Expr => 11,
            Self::Interjection => 12,
            Self::Pronoun => 13,
            Self::Numeric => 15,
            Self::Unclassified => 16,
            Self::Intransitive => 17,
            Self::Transitive => 18,
        }
    }
}

impl PartOfSpeech {
    /// Converts a `PartOfSpeech` tag to `PosSimple`
    pub fn to_pos_simple(&self) -> Vec<PosSimple> {
        let simple = match *self {
            PartOfSpeech::Adjective(_) | PartOfSpeech::AuxilaryAdj => PosSimple::Adjective,
            PartOfSpeech::Adverb | PartOfSpeech::AdverbTo => PosSimple::Adverb,
            PartOfSpeech::Auxilary => PosSimple::Auxilary,
            PartOfSpeech::Conjungation => PosSimple::Conjungation,
            PartOfSpeech::Counter => PosSimple::Counter,
            PartOfSpeech::Expr => PosSimple::Expr,
            PartOfSpeech::Interjection => PosSimple::Interjection,
            PartOfSpeech::Noun(_) => PosSimple::Noun,
            PartOfSpeech::Numeric => PosSimple::Numeric,
            PartOfSpeech::Pronoun => PosSimple::Pronoun,
            PartOfSpeech::Prefix => PosSimple::Prefix,
            PartOfSpeech::Suffix => PosSimple::Suffix,
            PartOfSpeech::Particle => PosSimple::Particle,
            PartOfSpeech::Unclassified => PosSimple::Unclassified,
            PartOfSpeech::Sfx => PosSimple::Sfx,
            PartOfSpeech::Verb(_) | PartOfSpeech::AuxilaryVerb => PosSimple::Verb,
        };

        if let PartOfSpeech::Verb(verb) = self {
            match verb {
                VerbType::Intransitive => vec![simple, PosSimple::Intransitive],
                VerbType::Transitive => vec![simple, PosSimple::Transitive],
                VerbType::Irregular(irr) => match irr {
                    IrregularVerb::NounOrAuxSuru => vec![simple, PosSimple::Noun],
                    _ => vec![simple],
                },
                _ => vec![simple],
            }
        } else {
            vec![simple]
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, PartialOrd, Ord, Eq, Deserialize, Hash)]
#[repr(u8)]
pub enum PartOfSpeech {
    // Adjectives
    Adjective(AdjectiveType),

    // Adverb
    Adverb,
    AdverbTo,

    // Auxilary
    Auxilary,
    AuxilaryAdj,
    AuxilaryVerb,

    // Other
    Conjungation,
    Counter,
    Expr,
    Interjection,

    Noun(NounType),

    Numeric,
    Pronoun,
    Prefix,
    Suffix,
    Particle,
    Unclassified,

    Sfx,

    // Verb
    Verb(VerbType),
}

impl PartOfSpeech {
    /// Returns true if [`self`] is a godan PartOfSpeech variant
    pub fn is_godan(&self) -> bool {
        if let PartOfSpeech::Verb(v) = self {
            matches!(v, VerbType::Godan(_))
        } else {
            false
        }
    }

    /// Returns true if [`self`] is an ichdan PartOfSpeech variant
    pub fn is_ichidan(&self) -> bool {
        if let PartOfSpeech::Verb(v) = self {
            match v {
                VerbType::Ichidan => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum VerbType {
    Nidan(NidanVerb),
    Yodan(VerbEnding),
    Godan(GodanVerbEnding),
    Irregular(IrregularVerb),
    Unspecified,
    Intransitive,
    Transitive,
    Ichidan,
    IchidanZuru,
    IchidanKureru,
    Kuru,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum AdjectiveType {
    PreNounVerb,
    /// I Adjective
    Keiyoushi,
    /// I Adjective conjugated like いい
    KeiyoushiYoiIi,
    Ku,
    Na,
    Nari,
    No,
    PreNoun,
    Shiku,
    Taru,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, PartialOrd, Ord, Eq, Deserialize, Hash)]
#[repr(u8)]
pub enum NounType {
    Normal,
    Adverbial,
    Prefix,
    Suffix,
    Temporal,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
pub enum IrregularVerb {
    Nu,
    Ru,
    NounOrAuxSuru,
    Suru,
    SuruSpecial,
    Su,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Eq, Hash)]
pub struct NidanVerb {
    class: VerbClass,
    ending: VerbEnding,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
pub enum VerbClass {
    Upper,
    Lower,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
pub enum VerbEnding {
    Bu,
    Dzu,
    Gu,
    Hu,
    Ku,
    Mu,
    Nu,
    Ru,
    Su,
    Tsu,
    U,
    Yu,
    Zu,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
pub enum GodanVerbEnding {
    Bu,
    Gu,
    Ku,
    Mu,
    Nu,
    Ru,
    Su,
    Tsu,
    U,

    Aru,
    USpecial,
    Uru,
    RuIrreg,
    IkuYuku,
}

impl Translatable for PartOfSpeech {
    fn get_id(&self) -> &'static str {
        match self {
            PartOfSpeech::Noun(noun_type) => noun_type.get_id(),
            PartOfSpeech::Sfx => "SoundFx",
            PartOfSpeech::Expr => "Expression",
            PartOfSpeech::Counter => "Counter",
            PartOfSpeech::Suffix => "Suffix",
            PartOfSpeech::Prefix => "Prefix",
            PartOfSpeech::Particle => "Particle",
            PartOfSpeech::Interjection => "Interjection",
            PartOfSpeech::Pronoun => "Pronoun",
            PartOfSpeech::Auxilary => "Auxilary",
            PartOfSpeech::Adjective(adj) => adj.get_id(),
            PartOfSpeech::Numeric => "Numeric",
            PartOfSpeech::AdverbTo => "Adverb-To",
            PartOfSpeech::Adverb => "Adverb",
            PartOfSpeech::Verb(verb) => verb.get_id(),
            PartOfSpeech::AuxilaryAdj => "Auxilary adjective",
            PartOfSpeech::AuxilaryVerb => "Auxilary Verb",
            PartOfSpeech::Conjungation => "Conjugation",
            PartOfSpeech::Unclassified => "Unclassified",
        }
    }

    fn gettext_custom(&self, dict: &TranslationDict, language: Option<Language>) -> String {
        match self {
            PartOfSpeech::Verb(verb) => verb.gettext_custom(dict, language),
            _ => self.gettext(dict, language).to_owned(),
        }
    }
}

impl Translatable for AdjectiveType {
    fn get_id(&self) -> &'static str {
        match self {
            AdjectiveType::PreNounVerb => "Noun or verb describing a noun",
            AdjectiveType::Keiyoushi => "I adjective",
            AdjectiveType::KeiyoushiYoiIi => "I adjective (conjugated like いい)",
            AdjectiveType::Ku => "Ku adjective",
            AdjectiveType::Na => "Na adjective",
            AdjectiveType::Nari => "Formal form of na adjective",
            AdjectiveType::No => "No adjective",
            AdjectiveType::PreNoun => "Pre noun adjective",
            AdjectiveType::Shiku => "Shiku adjective",
            AdjectiveType::Taru => "Taru adjective",
        }
    }
}

impl Translatable for NounType {
    fn get_id(&self) -> &'static str {
        match self {
            NounType::Normal => "Noun",
            NounType::Adverbial => "Noun adverbial",
            NounType::Prefix => "Prefix (noun)",
            NounType::Suffix => "Suffix (noun)",
            NounType::Temporal => "Temporal noun",
        }
    }
}

impl Translatable for VerbType {
    fn get_id(&self) -> &'static str {
        match *self {
            VerbType::Unspecified => "Unspecified verb",
            VerbType::Intransitive => "Intransitive verb",
            VerbType::Transitive => "Transitive verb",
            VerbType::Ichidan => "Ichidan verb",
            VerbType::IchidanZuru => "Ichidan zuru verb",
            VerbType::IchidanKureru => "Ichidan kureru verb",
            VerbType::Kuru => "Kuru verb",
            VerbType::Irregular(irregular) => irregular.get_id(),
            _ => "Godan verb",
        }
    }

    fn gettext_custom(&self, dict: &TranslationDict, language: Option<Language>) -> String {
        match self {
            VerbType::Irregular(i) => i.gettext_custom(dict, language),
            _ => self.gettext(dict, language).to_owned(),
        }
    }
}

impl Translatable for IrregularVerb {
    fn get_id(&self) -> &'static str {
        match self {
            IrregularVerb::Nu | IrregularVerb::Ru | IrregularVerb::Su => {
                "Irregular verb with {} ending"
            }
            IrregularVerb::NounOrAuxSuru => "Noun taking suru",
            IrregularVerb::Suru => "Suru verb",
            IrregularVerb::SuruSpecial => "Suru special",
        }
    }

    fn gettext_custom(&self, dict: &TranslationDict, language: Option<Language>) -> String {
        match self {
            IrregularVerb::Nu => self.gettext_fmt(dict, &["nu"], language),
            IrregularVerb::Ru => self.gettext_fmt(dict, &["ru"], language),
            IrregularVerb::Su => self.gettext_fmt(dict, &["su"], language),
            IrregularVerb::NounOrAuxSuru | IrregularVerb::Suru | IrregularVerb::SuruSpecial => {
                self.gettext(dict, language).to_owned()
            }
        }
    }
}

/// VerbType into String
impl Into<String> for VerbType {
    fn into(self) -> String {
        match self {
            VerbType::Nidan(nidan) => {
                let n: String = nidan.into();
                format!("{}{}", "v2", n)
            }
            VerbType::Yodan(yodan) => {
                let y: String = yodan.into();
                format!("{}{}", "v4", y)
            }
            VerbType::Godan(godan) => {
                let g: String = godan.into();
                format!("{}{}", "v5", g)
            }
            VerbType::Irregular(irreg) => irreg.into(),
            VerbType::Ichidan => "v1".to_owned(),
            VerbType::IchidanKureru => "v1-s".to_owned(),
            VerbType::Transitive => "vt".to_owned(),
            VerbType::Intransitive => "vi".to_owned(),
            VerbType::Kuru => "vk".to_owned(),
            VerbType::IchidanZuru => "vz".to_owned(),
            VerbType::Unspecified => "v-unspec".to_owned(),
        }
    }
}

/// Implement TryFrom for VerbType
impl TryFrom<&str> for VerbType {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 2 || value[..1] != *"v" {
            return Err(error::Error::Undefined);
        }

        Ok(match &value[1..2] {
            "1" => match value {
                "v1" => VerbType::Ichidan,
                "v1-s" => VerbType::IchidanKureru,
                _ => return Err(error::Error::Undefined),
            },
            "2" => VerbType::Nidan(NidanVerb::try_from(value)?), // Nidan
            "4" => VerbType::Yodan(VerbEnding::try_from(&value[2..3])?), // Yodan
            "5" => VerbType::Godan(GodanVerbEnding::try_from(&value[2..])?), // Godan
            _ => match value {
                "vi" => VerbType::Intransitive,
                "vt" => VerbType::Transitive,
                "v-unspec" => VerbType::Unspecified,
                "vz" => VerbType::IchidanZuru,
                "vk" => VerbType::Kuru,
                _ => VerbType::Irregular(IrregularVerb::try_from(value)?),
            },
        })
    }
}

impl TryFrom<&str> for IrregularVerb {
    type Error = error::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "vn" => IrregularVerb::Nu,
            "vr" => IrregularVerb::Ru,
            "vs" => IrregularVerb::NounOrAuxSuru,
            "vs-i" => IrregularVerb::Suru,
            "vs-s" => IrregularVerb::SuruSpecial,
            "vs-c" => IrregularVerb::Su,
            _ => return Err(error::Error::Undefined),
        })
    }
}

/// IrregularVerb into String
impl Into<String> for IrregularVerb {
    fn into(self) -> String {
        match self {
            IrregularVerb::Nu => "vn",
            IrregularVerb::Ru => "vr",
            IrregularVerb::NounOrAuxSuru => "vs",
            IrregularVerb::Suru => "vs-i",
            IrregularVerb::SuruSpecial => "vs-s",
            IrregularVerb::Su => "vs-c",
        }
        .to_string()
    }
}

/// GodanVerbEnding into String
impl Into<String> for GodanVerbEnding {
    fn into(self) -> String {
        match self {
            GodanVerbEnding::Aru => "aru",
            GodanVerbEnding::USpecial => "u-s",
            GodanVerbEnding::Uru => "uru",
            GodanVerbEnding::RuIrreg => "r-i",
            GodanVerbEnding::IkuYuku => "k-s",
            GodanVerbEnding::Bu => "b",
            GodanVerbEnding::Ku => "k",
            GodanVerbEnding::Gu => "g",
            GodanVerbEnding::Nu => "n",
            GodanVerbEnding::Mu => "m",
            GodanVerbEnding::Ru => "r",
            GodanVerbEnding::Su => "s",
            GodanVerbEnding::Tsu => "t",
            GodanVerbEnding::U => "u",
        }
        .to_string()
    }
}

/// Implement TryFrom for VerbEnding
impl TryFrom<&str> for GodanVerbEnding {
    type Error = error::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "aru" => GodanVerbEnding::Aru,
            "u-s" => GodanVerbEnding::USpecial,
            "uru" => GodanVerbEnding::Uru,
            "r-i" => GodanVerbEnding::RuIrreg,
            "k-s" => GodanVerbEnding::IkuYuku,
            _ => match &value[0..1] {
                "b" => GodanVerbEnding::Bu,
                "k" => GodanVerbEnding::Ku,
                "g" => GodanVerbEnding::Gu,
                "n" => GodanVerbEnding::Nu,
                "m" => GodanVerbEnding::Mu,
                "r" => GodanVerbEnding::Ru,
                "s" => GodanVerbEnding::Su,
                "t" => GodanVerbEnding::Tsu,
                "u" => GodanVerbEnding::U,
                _ => return Err(error::Error::Undefined),
            },
        })
    }
}

/// VerbEnding into String
impl Into<String> for VerbEnding {
    fn into(self) -> String {
        match self {
            VerbEnding::Bu => "b",
            VerbEnding::Dzu => "d",
            VerbEnding::Gu => "g",
            VerbEnding::Hu => "h",
            VerbEnding::Ku => "k",
            VerbEnding::Mu => "m",
            VerbEnding::Nu => "n",
            VerbEnding::Ru => "r",
            VerbEnding::Su => "s",
            VerbEnding::Tsu => "t",
            VerbEnding::U => "w",
            VerbEnding::Yu => "y",
            VerbEnding::Zu => "z",
        }
        .to_string()
    }
}

/// Implement TryFrom for VerbEnding
impl TryFrom<&str> for VerbEnding {
    type Error = error::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "b" => VerbEnding::Bu,
            "d" => VerbEnding::Dzu,
            "g" => VerbEnding::Gu,
            "h" => VerbEnding::Hu,
            "k" => VerbEnding::Ku,
            "m" => VerbEnding::Mu,
            "n" => VerbEnding::Nu,
            "r" => VerbEnding::Ru,
            "s" => VerbEnding::Su,
            "t" => VerbEnding::Tsu,
            "w" => VerbEnding::U,
            "y" => VerbEnding::Yu,
            "z" => VerbEnding::Zu,
            _ => return Err(error::Error::Undefined),
        })
    }
}

/// NidanVerb into String
impl Into<String> for NidanVerb {
    fn into(self) -> String {
        let class = match self.class {
            VerbClass::Upper => "k",
            VerbClass::Lower | VerbClass::None => "s",
        };
        let ending: String = self.ending.into();
        format!("{}-{}", ending, class)
    }
}

/// Implement TryFrom for NidanVerb
impl TryFrom<&str> for NidanVerb {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() < 3 || value[..1] != *"v" {
            return Err(error::Error::Undefined);
        }

        if value == "v2a-s" {
            return Ok(NidanVerb {
                ending: VerbEnding::U,
                class: VerbClass::None,
            });
        }

        let class: VerbClass = match &value[4..5] {
            "k" => VerbClass::Upper,
            "s" => VerbClass::Lower,
            _ => return Err(error::Error::Undefined),
        };

        let ending = VerbEnding::try_from(&value[2..3])?;

        Ok(NidanVerb { class, ending })
    }
}

/// NounType into String
impl Into<String> for NounType {
    fn into(self) -> String {
        match self {
            NounType::Normal => "n",
            NounType::Adverbial => "n-adv",
            NounType::Prefix => "n-pref",
            NounType::Suffix => "n-suf",
            NounType::Temporal => "n-t",
        }
        .to_string()
    }
}

/// Implement TryFrom for NounType
impl TryFrom<&str> for NounType {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match &value[2..] {
            "adv" => NounType::Adverbial,
            "pref" => NounType::Prefix,
            "suf" => NounType::Suffix,
            "t" => NounType::Temporal,
            _ => return Err(error::Error::Undefined),
        })
    }
}

impl Into<String> for AdjectiveType {
    fn into(self) -> String {
        match self {
            AdjectiveType::PreNounVerb => "adj-f",
            AdjectiveType::Keiyoushi => "adj-i",
            AdjectiveType::KeiyoushiYoiIi => "adj-ix",
            AdjectiveType::Ku => "adj-ku",
            AdjectiveType::Na => "adj-na",
            AdjectiveType::Nari => "adj-nari",
            AdjectiveType::No => "adj-no",
            AdjectiveType::PreNoun => "adj-pn",
            AdjectiveType::Shiku => "adj-shiku",
            AdjectiveType::Taru => "adj-t",
        }
        .to_string()
    }
}

/// Implement TryFrom for AdjectiveType
impl TryFrom<&str> for AdjectiveType {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value[4..].as_ref() {
            "f" => AdjectiveType::PreNounVerb,
            "i" => AdjectiveType::Keiyoushi,
            "ix" => AdjectiveType::KeiyoushiYoiIi,
            "ku" => AdjectiveType::Ku,
            "na" => AdjectiveType::Na,
            "nari" => AdjectiveType::Nari,
            "no" => AdjectiveType::No,
            "pn" => AdjectiveType::PreNoun,
            "shiku" => AdjectiveType::Shiku,
            "t" => AdjectiveType::Taru,
            _ => return Err(error::Error::Undefined),
        })
    }
}

impl Into<String> for PartOfSpeech {
    fn into(self) -> String {
        if let PartOfSpeech::Noun(noun) = self {
            return noun.into();
        }

        match self {
            PartOfSpeech::Adjective(adj) => adj.into(),
            PartOfSpeech::Noun(noun) => noun.into(),
            PartOfSpeech::Verb(verb) => verb.into(),
            _ => match self {
                PartOfSpeech::Pronoun => "pn",
                PartOfSpeech::Adverb => "adv",
                PartOfSpeech::Auxilary => "aux",
                PartOfSpeech::Counter => "ctr",
                PartOfSpeech::Conjungation => "conj",
                PartOfSpeech::Expr => "exp",
                PartOfSpeech::Interjection => "int",
                PartOfSpeech::Numeric => "num",
                PartOfSpeech::Particle => "prt",
                PartOfSpeech::Suffix => "suf",
                PartOfSpeech::Unclassified => "unc",
                PartOfSpeech::AdverbTo => "adv-to",
                PartOfSpeech::AuxilaryAdj => "aux-adj",
                PartOfSpeech::AuxilaryVerb => "aux-v",
                PartOfSpeech::Prefix => "pref",
                PartOfSpeech::Sfx => "sfx",
                _ => unreachable!(), // already checked above
            }
            .to_string(),
        }
    }
}

/// Implement TryFrom for PartOfSpeech
impl TryFrom<&str> for PartOfSpeech {
    type Error = error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "n" => PartOfSpeech::Noun(NounType::Normal),
            "pn" => PartOfSpeech::Pronoun,
            "sfx" => PartOfSpeech::Sfx,
            "adv" => PartOfSpeech::Adverb,
            "aux" => PartOfSpeech::Auxilary,
            "ctr" => PartOfSpeech::Counter,
            "exp" => PartOfSpeech::Expr,
            "int" => PartOfSpeech::Interjection,
            "num" => PartOfSpeech::Numeric,
            "prt" => PartOfSpeech::Particle,
            "conj" => PartOfSpeech::Conjungation,
            "suf" => PartOfSpeech::Suffix,
            "unc" => PartOfSpeech::Unclassified,
            "adv-to" => PartOfSpeech::AdverbTo,
            "aux-adj" => PartOfSpeech::AuxilaryAdj,
            "aux-v" => PartOfSpeech::AuxilaryVerb,
            "pref" => PartOfSpeech::Prefix,
            _ => {
                if value.starts_with("n-") {
                    return Ok(PartOfSpeech::Noun(NounType::try_from(value)?));
                }

                if value.starts_with("adj") {
                    return Ok(PartOfSpeech::Adjective(AdjectiveType::try_from(value)?));
                }

                if value.starts_with('v') {
                    return Ok(PartOfSpeech::Verb(VerbType::try_from(value)?));
                }

                return Err(error::Error::Undefined);
            }
        })
    }
}

// Part of speech tests
#[cfg(test)]
mod test {
    use super::*;
    use error::Error;
    use std::convert::TryInto;

    #[test]
    fn test_adv() {
        let pos: Result<PartOfSpeech, Error> = "adv".try_into();
        assert!(pos.is_ok());
        assert_eq!(pos.unwrap(), PartOfSpeech::Adverb);
    }

    #[test]
    fn test_adj() {
        let start_str = "adj-no";
        let pos: Result<PartOfSpeech, Error> = start_str.try_into();
        assert!(pos.is_ok());
        let pos = pos.unwrap();
        assert_eq!(pos, PartOfSpeech::Adjective(AdjectiveType::No));
        let s: String = pos.into();
        assert_eq!(start_str, s);
    }
}
/*
    #[test]
    fn test_intransitive() {
        let pos: Result<PartOfSpeech, Error> = "vi".try_into();
        assert_eq!(pos, Ok(PartOfSpeech::Verb(VerbType::Intransitive)));
    }

    #[test]
    fn test_irregular() {
        let pos: Result<PartOfSpeech, Error> = "vr".try_into();
        assert_eq!(
            pos,
            Ok(PartOfSpeech::Verb(VerbType::Irregular(IrregularVerb::Ru)))
        );
    }

    #[test]
    fn test_3_err() {
        let pos: Result<PartOfSpeech, Error> = "ads".try_into();
        assert_eq!(pos, Err(Error::Undefined));
    }

    #[test]
    fn test_empty() {
        let pos: Result<PartOfSpeech, Error> = "".try_into();
        assert_eq!(pos, Err(Error::Undefined));
    }

    #[test]
    fn test_adjective() {
        let pos: Result<AdjectiveType, Error> = "adj-f".try_into();
        assert_eq!(pos, Ok(AdjectiveType::PreNounVerb));
    }

    #[test]
    fn test_adjective_fail() {
        let pos: Result<AdjectiveType, Error> = "adjku".try_into();
        assert_eq!(pos, Err(Error::Undefined));
    }

    #[test]
    fn test_adjective_2() {
        let pos: Result<AdjectiveType, Error> = "adj-shiku".try_into();
        assert_eq!(pos, Ok(AdjectiveType::Shiku));
    }

    #[test]
    fn test_noun() {
        let pos: Result<NounType, Error> = "n-adv".try_into();
        assert_eq!(pos, Ok(NounType::Adverbial));
    }

    #[test]
    fn test_noun_fail() {
        let pos: Result<NounType, Error> = "n-eeee".try_into();
        assert_eq!(pos, Err(Error::Undefined));
    }

    #[test]
    fn test_noun_2() {
        let pos: Result<NounType, Error> = "n-suf".try_into();
        assert_eq!(pos, Ok(NounType::Suffix));
    }

    #[test]
    fn test_nidan() {
        let pos: Result<NidanVerb, Error> = "v2b-k".try_into();
        assert_eq!(
            pos,
            Ok(NidanVerb {
                class: VerbClass::Upper,
                ending: VerbEnding::Bu,
            })
        );
    }

    #[test]
    fn test_yodan() {
        let pos: Result<VerbType, Error> = "v4m".try_into();
        assert_eq!(pos, Ok(VerbType::Yodan(VerbEnding::Mu)));
    }

    #[test]
    fn test_godan_1() {
        let pos: Result<VerbType, Error> = "v5b".try_into();
        assert_eq!(pos, Ok(VerbType::Godan(GodanVerbEnding::Bu)));
    }

    #[test]
    fn test_godan_2() {
        let pos: Result<VerbType, Error> = "v5aru".try_into();
        assert_eq!(pos, Ok(VerbType::Godan(GodanVerbEnding::Aru)));
    }

    #[test]
    fn test_godan_3() {
        let pos: Result<VerbType, Error> = "v5uru".try_into();
        assert_eq!(pos, Ok(VerbType::Godan(GodanVerbEnding::Uru)));
    }

    #[test]
    fn test_godan_4() {
        let pos: Result<VerbType, Error> = "v5t".try_into();
        assert_eq!(pos, Ok(VerbType::Godan(GodanVerbEnding::Tsu)));
    }

    #[test]
    fn test_ru_irreg() {
        let pos: Result<GodanVerbEnding, Error> = "r-i".try_into();
        assert_eq!(pos, Ok(GodanVerbEnding::RuIrreg));
    }

    #[test]
    fn test_godan_5() {
        let pos: Result<VerbType, Error> = "v5u-s".try_into();
        assert_eq!(pos, Ok(VerbType::Godan(GodanVerbEnding::USpecial)));
    }

    #[test]
    fn test_consistency() {
        let items = vec![
            PartOfSpeech::Adverb,
            PartOfSpeech::Verb(VerbType::Godan(GodanVerbEnding::RuIrreg)),
            PartOfSpeech::Verb(VerbType::Godan(GodanVerbEnding::IkuYuku)),
            PartOfSpeech::Verb(VerbType::Yodan(VerbEnding::Ku)),
            PartOfSpeech::Verb(VerbType::Yodan(VerbEnding::Ru)),
            PartOfSpeech::Verb(VerbType::Nidan(NidanVerb {
                ending: VerbEnding::Nu,
                class: VerbClass::Upper,
            })),
            PartOfSpeech::Verb(VerbType::Nidan(NidanVerb {
                ending: VerbEnding::Mu,
                class: VerbClass::Lower,
            })),
            PartOfSpeech::Verb(VerbType::Nidan(NidanVerb {
                ending: VerbEnding::Ku,
                class: VerbClass::Lower,
            })),
            PartOfSpeech::Verb(VerbType::Ichidan),
            PartOfSpeech::Verb(VerbType::Irregular(IrregularVerb::NounOrAuxSuru)),
            PartOfSpeech::Verb(VerbType::Irregular(IrregularVerb::Su)),
            PartOfSpeech::Verb(VerbType::Irregular(IrregularVerb::Suru)),
            PartOfSpeech::Verb(VerbType::Irregular(IrregularVerb::SuruSpecial)),
            PartOfSpeech::Verb(VerbType::Unspecified),
            PartOfSpeech::Noun(NounType::Normal),
            PartOfSpeech::Noun(NounType::Adverbial),
            PartOfSpeech::Noun(NounType::Temporal),
            PartOfSpeech::Interjection,
            PartOfSpeech::Conjungation,
            PartOfSpeech::Unclassified,
            PartOfSpeech::Counter,
            PartOfSpeech::Particle,
        ];

        for item in items {
            let to_str: String = item.clone().into();
            println!("parsing: {:?} to_str: '{}'", item, to_str.as_str());
            let back: Result<PartOfSpeech, Error> = to_str.as_str().try_into();
            assert!(back.is_ok());
            assert_eq!(back.unwrap(), item);
        }
    }
}
*/
