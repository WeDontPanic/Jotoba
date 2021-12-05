#[cfg(feature = "jotoba_intern")]
use localization::traits::Translatable;
use strum_macros::{AsRefStr, EnumString};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize, Deserialize, Hash)]
#[repr(u8)]
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
    #[strum(serialize = "doc")]
    Document,
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
    #[strum(serialize = "group")]
    Group,
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
    #[strum(serialize = "form")]
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

#[cfg(feature = "jotoba_intern")]
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
            Misc::Document => "Document",
            Misc::Event => "Event",
            Misc::FamiliarLanguage => "Familiar language",
            Misc::FemaleTermOrLanguage => "Female term/language",
            Misc::Fiction => "Fiction",
            Misc::GivenName => "Given name",
            Misc::Group => "Group",
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
