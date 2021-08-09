use localization::traits::Translatable;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize, Deserialize)]
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
}

impl NameType {
    pub fn is_gender(&self) -> bool {
        matches!(self, Self::Female | Self::Male)
    }
}

impl Translatable for NameType {
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
        }
    }
}
