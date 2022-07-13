#[cfg(feature = "jotoba_intern")]
use localization::traits::Translatable;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize, Deserialize, Hash, EnumIter,
)]
#[repr(u8)]
pub enum Misc {
    #[strum(serialize = "abbr", serialize = "abbreviation")]
    Abbreviation,
    #[strum(serialize = "arch", serialize = "archaism")]
    Archaism,
    #[strum(serialize = "char")]
    Character,
    #[strum(serialize = "chn", serialize = "childrenslanguage")]
    ChildrensLanguage,
    #[strum(serialize = "col", serialize = "colloquialism")]
    Colloquialism,
    #[strum(serialize = "company")]
    CompanyName,
    #[strum(serialize = "creat")]
    Creature,
    #[strum(serialize = "dated")]
    DatedTerm,
    #[strum(serialize = "dei")]
    Deity,
    #[strum(serialize = "derog", serialize = "derogatory")]
    Derogatory,
    #[strum(serialize = "doc")]
    Document,
    #[strum(serialize = "ev")]
    Event,
    #[strum(serialize = "fam", serialize = "familiarlanguage")]
    FamiliarLanguage,
    #[strum(serialize = "fem", serialize = "femaleterm")]
    FemaleTermOrLanguage,
    #[strum(serialize = "fict", serialize = "fiction")]
    Fiction,
    #[strum(serialize = "given")]
    GivenName,
    #[strum(serialize = "group")]
    Group,
    #[strum(serialize = "hist", serialize = "Historical")]
    HistoricalTerm,
    #[strum(serialize = "hon", serialize = "honorific")]
    HonorificLanguage,
    #[strum(serialize = "hum", serialize = "humblelanguage")]
    HumbleLanguage,
    #[strum(serialize = "id", serialize = "idomatic")]
    IdiomaticExpression,
    #[strum(serialize = "joc")]
    JocularHumorousTerm,
    #[strum(serialize = "leg", serialize = "legend")]
    Legend,
    #[strum(serialize = "form", serialize = "formal")]
    LiteraryOrFormalTerm,
    #[strum(serialize = "m-sl", serialize = "mangaslang")]
    MangaSlang,
    #[strum(serialize = "male", serialize = "maleterm")]
    MaleTermOrLanguage,
    #[strum(serialize = "myth")]
    Mythology,
    #[strum(serialize = "net-sl", serialize = "internetslang")]
    InternetSlang,
    #[strum(serialize = "obj", serialize = "object")]
    Object,
    #[strum(serialize = "obs", serialize = "obsolete")]
    ObsoleteTerm,
    #[strum(serialize = "obsc", serialize = "obscure")]
    ObscureTerm,
    #[strum(serialize = "on-mim", serialize = "onomatopoeic")]
    OnomatopoeicOrMimeticWord,
    #[strum(serialize = "organization")]
    OrganizationName,
    #[strum(serialize = "oth", serialize = "other")]
    Other,
    #[strum(serialize = "person", serialize = "personname")]
    Personname,
    #[strum(serialize = "place", serialize = "placename")]
    PlaceName,
    #[strum(serialize = "poet", serialize = "poeticalterm")]
    PoeticalTerm,
    #[strum(serialize = "pol", serialize = "politelanguage")]
    PoliteLanguage,
    #[strum(serialize = "product", serialize = "productname")]
    ProductName,
    #[strum(serialize = "proverb")]
    Proverb,
    #[strum(serialize = "quote", serialize = "quotation")]
    Quotation,
    #[strum(serialize = "rare")]
    Rare,
    #[strum(serialize = "relig")]
    Religion,
    #[strum(serialize = "sens", serialize = "sensitive")]
    Sensitive,
    #[strum(serialize = "serv")]
    Service,
    #[strum(serialize = "sl", serialize = "slang")]
    Slang,
    #[strum(serialize = "station")]
    RailwayStation,
    #[strum(serialize = "surname")]
    FamilyOrSurname,
    #[strum(serialize = "uk", serialize = "usuallykana")]
    UsuallyWrittenInKana,
    #[strum(serialize = "unclass")]
    UnclassifiedName,
    #[strum(serialize = "vulg", serialize = "vulgar")]
    VulgarExpressionOrWord,
    #[strum(serialize = "work", serialize = "artwork")]
    ArtWork,
    #[strum(serialize = "X", serialize = "rude")]
    RudeOrXRatedTerm,
    #[strum(serialize = "yoji", serialize = "yojijukugo")]
    Yojijukugo,
}

#[cfg(feature = "jotoba_intern")]
impl Translatable for Misc {
    fn get_id(&self) -> &'static str {
        match self {
            Misc::Abbreviation => "Abbreviation",
            Misc::Archaism => "Archaism",
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

impl Misc {
    #[inline]
    pub fn iter() -> impl Iterator<Item = Misc> {
        <Misc as IntoEnumIterator>::iter()
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}
