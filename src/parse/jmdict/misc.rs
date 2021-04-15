use std::{io::Write, str::FromStr};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Text,
    types::{FromSql, ToSql},
};

use strum_macros::{AsRefStr, EnumString};

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
#[sql_type = "Text"]
pub enum Misc {
    #[strum(serialize = "abbr")]
    Abbreviation,
    #[strum(serialize = "arch")]
    Archaism,
    #[strum(serialize = "char")]
    Character,
    #[strum(serialize = "chn")]
    ChildrensLanguage,
    #[strum(serialize = "col")]
    Colloquialism,
    #[strum(serialize = "company")]
    CompanyName,
    #[strum(serialize = "creat")]
    Creature,
    #[strum(serialize = "dated")]
    DatedTerm,
    #[strum(serialize = "dei")]
    Deity,
    #[strum(serialize = "derog")]
    Derogatory,
    #[strum(serialize = "ev")]
    Event,
    #[strum(serialize = "fam")]
    FamiliarLanguage,
    #[strum(serialize = "fem")]
    FemaleTermOrLanguage,
    #[strum(serialize = "fict")]
    Fiction,
    #[strum(serialize = "given")]
    GivenName,
    #[strum(serialize = "hist")]
    HistoricalTerm,
    #[strum(serialize = "hon")]
    HonorificLanguage,
    #[strum(serialize = "hum")]
    HumbleLanguage,
    #[strum(serialize = "id")]
    IdiomaticExpression,
    #[strum(serialize = "joc")]
    JocularHumorousTerm,
    #[strum(serialize = "leg")]
    Legend,
    #[strum(serialize = "litf")]
    LiteraryOrFormalTerm,
    #[strum(serialize = "m-sl")]
    MangaSlang,
    #[strum(serialize = "male")]
    MaleTermOrLanguage,
    #[strum(serialize = "myth")]
    Mythology,
    #[strum(serialize = "net-sl")]
    InternetSlang,
    #[strum(serialize = "obj")]
    Object,
    #[strum(serialize = "obs")]
    ObsoleteTerm,
    #[strum(serialize = "obsc")]
    ObscureTerm,
    #[strum(serialize = "on-mim")]
    OnomatopoeicOrMimeticWord,
    #[strum(serialize = "organization")]
    OrganizationName,
    #[strum(serialize = "oth")]
    Other,
    #[strum(serialize = "person")]
    Personname,
    #[strum(serialize = "place")]
    PlaceName,
    #[strum(serialize = "poet")]
    PoeticalTerm,
    #[strum(serialize = "pol")]
    PoliteLanguage,
    #[strum(serialize = "product")]
    ProductName,
    #[strum(serialize = "proverb")]
    Proverb,
    #[strum(serialize = "quote")]
    Quotation,
    #[strum(serialize = "rare")]
    Rare,
    #[strum(serialize = "relig")]
    Religion,
    #[strum(serialize = "sens")]
    Sensitive,
    #[strum(serialize = "serv")]
    Service,
    #[strum(serialize = "sl")]
    Slang,
    #[strum(serialize = "station")]
    RailwayStation,
    #[strum(serialize = "surname")]
    FamilyOrSurname,
    #[strum(serialize = "uk")]
    UsuallyWrittenInKana,
    #[strum(serialize = "unclass")]
    UnclassifiedName,
    #[strum(serialize = "vulg")]
    VulgarExpressionOrWord,
    #[strum(serialize = "work")]
    ArtWork,
    #[strum(serialize = "X")]
    RudeOrXRatedTerm,
    #[strum(serialize = "yoji")]
    Yojijukugo,
}

impl Into<String> for Misc {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}

impl ToSql<Text, Pg> for Misc {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <&str as ToSql<Text, Pg>>::to_sql(&self.as_ref(), out)
    }
}

impl FromSql<Text, Pg> for Misc {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::from_str(
            (<String as FromSql<Text, Pg>>::from_sql(bytes)?).as_str(),
        )?)
    }
}
