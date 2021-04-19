#![allow(clippy::from_over_into)]
use std::{
    convert::{TryFrom, TryInto},
    io::Write,
};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Text,
    types::{FromSql, ToSql},
};

use crate::error;

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy)]
#[sql_type = "Text"]
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
    Copula,
    Counter,
    Expr,
    Interjection,

    Noun(NounType),

    Nummeric,
    Pronoun,
    Prefix,
    Suffix,
    Particle,
    Unclassified,

    SFX,

    // Verb
    Verb(VerbType),
}

impl ToSql<Text, Pg> for PartOfSpeech {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s: String = (*self).into();
        <&str as ToSql<Text, Pg>>::to_sql(&s.as_str(), out)
    }
}

impl FromSql<Text, Pg> for PartOfSpeech {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(<String as FromSql<Text, Pg>>::from_sql(bytes)?
            .as_str()
            .try_into()?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AdjectiveType {
    PreNounVerb,
    Keiyoushi,
    KeiyoushiYoiIi,
    KAri,
    Ku,
    Na,
    Nari,
    No,
    PreNoun,
    Shiku,
    Taru,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NounType {
    Normal,
    Adverbial,
    Proper,
    Prefix,
    Suffix,
    Temporal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IrregularVerb {
    Nu,
    Ru,
    NounOrAuxSuru,
    Suru,
    SuruSpecial,
    Su,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NidanVerb {
    class: VerbClass,
    ending: VerbEnding,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerbClass {
    Upper,
    Lower,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl PartOfSpeech {
    pub fn humanized(&self) -> String {
        match *self {
            Self::Noun(noun_type) => noun_type.humanized(),
            Self::SFX => "SoundFx".to_string(),
            Self::Expr => "Expression".to_string(),
            Self::Counter => "Counter".to_string(),
            Self::Suffix => "Suffix".to_string(),
            Self::Prefix => "Prefix".to_string(),
            Self::Particle => "Particle".to_string(),
            Self::Interjection => "Interjection".to_string(),
            Self::Pronoun => "Pronoun".to_string(),
            Self::Auxilary => "Auxilary".to_string(),
            Self::Adjective(adj) => adj.humanized(),
            Self::Nummeric => "Nummeric".to_string(),
            Self::AdverbTo => "Adverb-To".to_string(),
            Self::Adverb => "Adverb".to_string(),
            Self::Verb(verb) => verb.humanized(),
            _ => (*self).into(),
        }
    }
}

impl VerbType {
    fn humanized(&self) -> String {
        match *self {
            VerbType::Irregular(irreg) => irreg.humanize(),
            VerbType::Unspecified => "Unspecified verb".to_string(),
            VerbType::Intransitive => "Intransitive verb".to_string(),
            VerbType::Transitive => "Transitive verb".to_string(),
            VerbType::Ichidan => "Ichidan verb".to_string(),
            VerbType::IchidanZuru => "Ichidan zuru verb".to_string(),
            VerbType::IchidanKureru => "Ichidan kureru verb".to_string(),
            VerbType::Kuru => "Kuru verb".to_string(),
            // TODO do other
            _ => format!("{:?}", self),
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

impl IrregularVerb {
    fn humanize(&self) -> String {
        match *self {
            IrregularVerb::Nu => "Nu verb",
            IrregularVerb::Ru => "Ru verb",
            IrregularVerb::Suru | IrregularVerb::NounOrAuxSuru => "Suru verb",
            IrregularVerb::SuruSpecial => "Suru verb (special class)",
            IrregularVerb::Su => "Su verb",
        }
        .to_string()
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
            NounType::Proper => "n-pr",
            NounType::Prefix => "n-pref",
            NounType::Suffix => "n-suf",
            NounType::Temporal => "n-t",
        }
        .to_string()
    }
}

impl NounType {
    fn humanized(&self) -> String {
        match *self {
            NounType::Normal => "Noun",
            NounType::Adverbial => "Noun adverbial",
            NounType::Proper => "Noun (proper)",
            NounType::Prefix => "Noun (prefix)",
            NounType::Suffix => "Noun (suffix)",
            NounType::Temporal => "Temporal noun",
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
            "pr" => NounType::Proper,
            "pref" => NounType::Prefix,
            "suf" => NounType::Suffix,
            "t" => NounType::Temporal,
            _ => return Err(error::Error::Undefined),
        })
    }
}

impl AdjectiveType {
    fn humanized(&self) -> String {
        match *self {
            Self::Na => "Na adjective".to_string(),
            Self::No => "No adjective".to_string(),
            Self::PreNounVerb => "Prenoun adjective".to_string(),
            Self::Keiyoushi => "I adjective".to_string(),
            // TODO implement properly
            _ => format!("{:?}", *self),
        }
    }
}

impl Into<String> for AdjectiveType {
    fn into(self) -> String {
        match self {
            AdjectiveType::PreNounVerb => "adj-f",
            AdjectiveType::Keiyoushi => "adj-i",
            AdjectiveType::KeiyoushiYoiIi => "adj-ix",
            AdjectiveType::KAri => "adj-kari",
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
            "kari" => AdjectiveType::KAri,
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
                PartOfSpeech::Copula => "cop",
                PartOfSpeech::Counter => "ctr",
                PartOfSpeech::Conjungation => "conj",
                PartOfSpeech::Expr => "exp",
                PartOfSpeech::Interjection => "int",
                PartOfSpeech::Nummeric => "num",
                PartOfSpeech::Particle => "prt",
                PartOfSpeech::Suffix => "suf",
                PartOfSpeech::Unclassified => "unc",
                PartOfSpeech::AdverbTo => "adv-to",
                PartOfSpeech::AuxilaryAdj => "aux-adj",
                PartOfSpeech::AuxilaryVerb => "aux-v",
                PartOfSpeech::Prefix => "pref",
                PartOfSpeech::SFX => "sfx",
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
            "sfx" => PartOfSpeech::SFX,
            "adv" => PartOfSpeech::Adverb,
            "aux" => PartOfSpeech::Auxilary,
            "cop" => PartOfSpeech::Copula,
            "ctr" => PartOfSpeech::Counter,
            "exp" => PartOfSpeech::Expr,
            "int" => PartOfSpeech::Interjection,
            "num" => PartOfSpeech::Nummeric,
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
    use crate::error::Error;
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
