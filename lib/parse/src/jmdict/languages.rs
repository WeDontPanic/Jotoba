use std::{convert::TryFrom, io::Write};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Integer,
    types::{FromSql, ToSql},
};
use postgres_types::{accepts, to_sql_checked};
use strum_macros::{AsRefStr, Display, EnumString};

use crate::error;

#[derive(
    AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Display, Hash, Eq,
)]
#[sql_type = "Integer"]
pub enum Language {
    #[strum(serialize = "eng", serialize = "en-US")]
    English,
    #[strum(serialize = "ger", serialize = "de-DE")]
    German,
    #[strum(serialize = "rus", serialize = "ru")]
    Russain,
    #[strum(serialize = "spa", serialize = "es-ES")]
    Spanish,
    #[strum(serialize = "swe", serialize = "sv-SE")]
    Swedish,
    #[strum(serialize = "fre", serialize = "fr-FR")]
    French,
    #[strum(serialize = "dut", serialize = "nl-NL")]
    Dutch,
    #[strum(serialize = "hun", serialize = "hu")]
    Hungarian,
    #[strum(serialize = "slv", serialize = "sl-SL", serialize = "svl")]
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
            0 => Self::English,
            1 => Self::German,
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
            Self::English => 0,
            Self::German => 1,
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

impl<'a> tokio_postgres::types::FromSql<'a> for Language {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Language::try_from(
            <i32 as tokio_postgres::types::FromSql>::from_sql(ty, raw)?,
        )?)
    }

    accepts!(INT4);
}

impl tokio_postgres::types::ToSql for Language {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let i: i32 = (*self).into();
        Ok(<i32 as tokio_postgres::types::ToSql>::to_sql(&i, ty, out)?)
    }

    accepts!(INT4);

    to_sql_checked!();
}
