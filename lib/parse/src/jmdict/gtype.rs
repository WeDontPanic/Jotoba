use std::{convert::TryFrom, io::Write};

use postgres_types::{accepts, to_sql_checked};
use strum_macros::{AsRefStr, EnumString};
use tokio_postgres::types::{FromSql, ToSql};

use crate::error;

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
pub enum GType {
    #[strum(serialize = "lit")]
    Literal,
    #[strum(serialize = "fig")]
    Figurative,
    #[strum(serialize = "expl")]
    Explanation,
}

impl TryFrom<i32> for GType {
    type Error = error::Error;
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        Ok(match i {
            0 => Self::Literal,
            1 => Self::Figurative,
            2 => Self::Explanation,
            _ => return Err(error::Error::ParseError),
        })
    }
}

impl Into<i32> for GType {
    fn into(self) -> i32 {
        match self {
            Self::Literal => 0,
            Self::Figurative => 1,
            Self::Explanation => 2,
        }
    }
}

impl<'a> FromSql<'a> for GType {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self::try_from(
            <i32 as tokio_postgres::types::FromSql>::from_sql(ty, raw)?,
        )?)
    }

    accepts!(INT4);
}

impl ToSql for GType {
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
