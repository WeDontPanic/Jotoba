use std::convert::TryFrom;

use localization::traits::Translatable;
use postgres_types::{accepts, to_sql_checked};
use strum_macros::{AsRefStr, EnumString};
use tokio_postgres::types::{FromSql, ToSql};

use crate::error;

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize)]
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

impl NameType {
    pub fn is_gender(&self) -> bool {
        matches!(self, Self::Female | Self::Male)
    }
}

impl TryFrom<i32> for NameType {
    type Error = crate::error::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => NameType::Company,
            1 => NameType::Female,
            2 => NameType::Male,
            3 => NameType::Given,
            4 => NameType::Organization,
            5 => NameType::Person,
            6 => NameType::Place,
            7 => NameType::Product,
            8 => NameType::RailwayStation,
            9 => NameType::Surname,
            10 => NameType::Unclassified,
            11 => NameType::Work,
            _ => return Err(error::Error::ParseError),
        })
    }
}

impl From<NameType> for i32 {
    fn from(nt: NameType) -> i32 {
        match nt {
            NameType::Company => 0,
            NameType::Female => 1,
            NameType::Male => 2,
            NameType::Given => 3,
            NameType::Organization => 4,
            NameType::Person => 5,
            NameType::Place => 6,
            NameType::Product => 7,
            NameType::RailwayStation => 8,
            NameType::Surname => 9,
            NameType::Unclassified => 10,
            NameType::Work => 11,
        }
    }
}

impl<'a> FromSql<'a> for NameType {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(NameType::try_from(
            <i32 as tokio_postgres::types::FromSql>::from_sql(ty, raw)?,
        )?)
    }

    accepts!(INT4);
}

impl ToSql for NameType {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let s: i32 = (*self).into();
        Ok(<i32 as ToSql>::to_sql(&s, ty, out)?)
    }

    accepts!(INT4);

    to_sql_checked!();
}
