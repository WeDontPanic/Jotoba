use std::{convert::TryFrom, io::Write};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Integer,
    types::{FromSql, ToSql},
};

use strum_macros::{AsRefStr, EnumString};

use crate::error;

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
#[sql_type = "Integer"]
pub enum GType {
    #[strum(serialize = "lit")]
    Literal,
    #[strum(serialize = "fig")]
    Figurative,
    #[strum(serialize = "expl")]
    Explanation,
}

impl ToSql<Integer, Pg> for GType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <i32 as ToSql<Integer, Pg>>::to_sql(&(*self).into(), out)
    }
}

impl FromSql<Integer, Pg> for GType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::try_from(<i32 as FromSql<Integer, Pg>>::from_sql(
            bytes,
        )?)?)
    }
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
