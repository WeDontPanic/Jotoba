use std::{io::Write, str::FromStr};

use localization::traits::Translatable;
use postgres_types::{accepts, to_sql_checked};
use strum_macros::{AsRefStr, EnumString};
use tokio_postgres::types::{FromSql, ToSql};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
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

impl Translatable for Misc {
    fn get_id(&self) -> &'static str {
        match self {
            Misc::Abbreviation => "Abbreviation",
            Misc::Archaism => "Anarchism",
            Misc::Character => "Character",
            Misc::ChildrensLanguage => "Childrens language",
            Misc::Colloquialism => "Colloquialism",
            Misc::CompanyName => "Company name",
            Misc::Creature => "Creature",
            Misc::DatedTerm => "Dated term",
            Misc::Deity => "Deity",
            Misc::Derogatory => "Derogatory",
            Misc::Event => "Event",
            Misc::FamiliarLanguage => "Familiar language",
            Misc::FemaleTermOrLanguage => "Female term/language",
            Misc::Fiction => "Fiction",
            Misc::GivenName => "Given name",
            Misc::HistoricalTerm => "Historical term",
            Misc::HonorificLanguage => "Honorific language",
            Misc::HumbleLanguage => "Humble language",
            Misc::IdiomaticExpression => "Idiomatic expression",
            Misc::JocularHumorousTerm => "Jocular humorous term",
            Misc::Legend => "Legend",
            Misc::LiteraryOrFormalTerm => "Literary/formal term",
            Misc::MangaSlang => "Manga slang",
            Misc::MaleTermOrLanguage => "Male term/language",
            Misc::Mythology => "Mythology",
            Misc::InternetSlang => "Internet slang",
            Misc::Object => "Object",
            Misc::ObsoleteTerm => "Obsolete term",
            Misc::ObscureTerm => "Obscure term",
            Misc::OnomatopoeicOrMimeticWord => "Onomatopoetic or mimetic word",
            Misc::OrganizationName => "Organization name",
            Misc::Other => "Other",
            Misc::Personname => "Person name",
            Misc::PlaceName => "Place name",
            Misc::PoeticalTerm => "Poetical term",
            Misc::PoliteLanguage => "Polite language",
            Misc::ProductName => "Product name",
            Misc::Proverb => "Proverb",
            Misc::Quotation => "Qutation",
            Misc::Rare => "Rare",
            Misc::Religion => "Religion",
            Misc::Sensitive => "Sensitive",
            Misc::Service => "Service",
            Misc::Slang => "Slang",
            Misc::RailwayStation => "Railway station",
            Misc::FamilyOrSurname => "Family or surname",
            Misc::UsuallyWrittenInKana => "Usually written in kana",
            Misc::UnclassifiedName => "Unclassified name",
            Misc::VulgarExpressionOrWord => "Vulgar expression/word",
            Misc::ArtWork => "Artwork",
            Misc::RudeOrXRatedTerm => "Rude/x-rated term",
            Misc::Yojijukugo => "Yojijukugo",
        }
    }
}

impl<'a> FromSql<'a> for Misc {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self::from_str(
            <String as tokio_postgres::types::FromSql>::from_sql(ty, raw)?.as_str(),
        )?)
    }

    accepts!(TEXT);
}

impl ToSql for Misc {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        Ok(<&str as ToSql>::to_sql(&self.as_ref(), ty, out)?)
    }

    accepts!(TEXT);

    to_sql_checked!();
}
