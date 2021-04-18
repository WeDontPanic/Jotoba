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
pub enum NameType {
    #[strum(serialize = "company")]
    Company,
    #[strum(serialize = "female")]
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

impl ToSql<Text, Pg> for NameType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <&str as ToSql<Text, Pg>>::to_sql(&self.as_ref(), out)
    }
}

impl FromSql<Text, Pg> for NameType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::from_str(
            (<String as FromSql<Text, Pg>>::from_sql(bytes)?).as_str(),
        )?)
    }
}
