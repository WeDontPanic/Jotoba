use std::{convert::TryFrom, io::Write};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Integer,
    types::{FromSql, ToSql},
};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::error;

#[derive(
    AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Display,
)]
#[sql_type = "Integer"]
pub enum Language {
    #[strum(serialize = "ger")]
    German,
    #[strum(serialize = "eng")]
    English,
    #[strum(serialize = "rus")]
    Russain,
    #[strum(serialize = "spa")]
    Spanish,
    #[strum(serialize = "swe")]
    Swedish,
    #[strum(serialize = "fre")]
    French,
    #[strum(serialize = "dut")]
    Dutch,
    #[strum(serialize = "hun")]
    Hungarian,
    #[strum(serialize = "slv")]
    Slovenian,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

impl ToSql<Integer, Pg> for Language {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <i32 as ToSql<Integer, Pg>>::to_sql(&(*self).into(), out)
    }
}

impl FromSql<Integer, Pg> for Language {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::try_from(<i32 as FromSql<Integer, Pg>>::from_sql(
            bytes,
        )?)?)
    }
}

impl TryFrom<i32> for Language {
    type Error = error::Error;
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        Ok(match i {
            0 => Self::German,
            1 => Self::English,
            2 => Self::Russain,
            3 => Self::Spanish,
            4 => Self::Swedish,
            5 => Self::French,
            6 => Self::Dutch,
            7 => Self::Hungarian,
            8 => Self::Slovenian,
            _ => return Err(error::Error::ParseError),
        })
    }
}

impl Into<i32> for Language {
    fn into(self) -> i32 {
        match self {
            Self::German => 0,
            Self::English => 1,
            Self::Russain => 2,
            Self::Spanish => 3,
            Self::Swedish => 4,
            Self::French => 5,
            Self::Dutch => 6,
            Self::Hungarian => 7,
            Self::Slovenian => 8,
        }
    }
}
