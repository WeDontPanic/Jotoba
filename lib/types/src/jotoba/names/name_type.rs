#[cfg(feature = "jotoba_intern")]
use localization::traits::Translatable;

use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, EnumString, Serialize, Deserialize, PartialEq, Hash)]
#[repr(u8)]
pub enum NameType {
    #[strum(serialize = "company")]
    Company,
    #[strum(serialize = "fem")]
    Female,
    #[strum(serialize = "masc")]
    Male,
    #[strum(serialize = "given")]
    Given,
    #[strum(serialize = "organization")]
    Organization,
    #[strum(serialize = "person")]
    Person,
    #[strum(serialize = "place")]
    Place,
    #[strum(serialize = "product")]
    Product,
    #[strum(serialize = "station")]
    RailwayStation,
    #[strum(serialize = "surname")]
    Surname,
    #[strum(serialize = "unclass")]
    Unclassified,
    #[strum(serialize = "work")]
    Work,
    #[strum(serialize = "char")]
    Character,
    #[strum(serialize = "creat")]
    Creature,
    #[strum(serialize = "dei")]
    Deity,
    #[strum(serialize = "doc")]
    Document,
    #[strum(serialize = "ev")]
    Event,
    #[strum(serialize = "fict")]
    Fiction,
    #[strum(serialize = "group")]
    Group,
    #[strum(serialize = "leg")]
    Legend,
    #[strum(serialize = "myth")]
    Mythology,
    #[strum(serialize = "obj")]
    Object,
    #[strum(serialize = "oth")]
    Other,
    #[strum(serialize = "relig")]
    Religion,
    #[strum(serialize = "serv")]
    Service,
}

impl NameType {
    #[inline]
    pub fn is_gender(&self) -> bool {
        matches!(self, Self::Female | Self::Male)
    }
}

#[cfg(feature = "jotoba_intern")]
impl Translatable for NameType {
    #[inline]
    fn get_id(&self) -> &'static str {
        match self {
            NameType::Company => "Company",
            NameType::Female => "Female",
            NameType::Male => "Male",
            NameType::Given => "Given name",
            NameType::Organization => "Organization",
            NameType::Person => "Persons name",
            NameType::Place => "Place",
            NameType::Product => "Product",
            NameType::RailwayStation => "(Railway)Station",
            NameType::Surname => "Surname",
            NameType::Unclassified => "Unknown",
            NameType::Work => "Art work",
            NameType::Character => "Character",
            NameType::Creature => "Creature",
            NameType::Deity => "Deity",
            NameType::Document => "Document",
            NameType::Event => "Event",
            NameType::Fiction => "Fiction",
            NameType::Group => "Group",
            NameType::Legend => "Legend",
            NameType::Mythology => "Mythology",
            NameType::Object => "Object",
            NameType::Other => "Other",
            NameType::Religion => "Religion",
            NameType::Service => "Service",
        }
    }
}
