use std::{convert::TryFrom, io::Write};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Integer,
    types::{FromSql, ToSql},
};

use localization::traits::Translatable;
use strum_macros::{AsRefStr, EnumString};

use crate::error;

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
#[sql_type = "Integer"]
pub enum Information {
    #[strum(serialize = "ateji")]
    Ateji,
    #[strum(serialize = "ik")]
    IrregularKana,
    #[strum(serialize = "iK")]
    IrregularKanji,
    #[strum(serialize = "io")]
    IrregularOkurigana,
    #[strum(serialize = "oK")]
    OutdatedKanji,
    #[strum(serialize = "ok")]
    OutdatedKana,
    #[strum(serialize = "gikun")]
    Gikun,
    #[strum(serialize = "uK")]
    UsuallyKana,
}

impl Translatable for Information {
    fn get_id(&self) -> &'static str {
        match self {
            Information::Ateji => "ateji",
            Information::IrregularKana => "irregular kana",
            Information::IrregularKanji => "irregular kanji",
            Information::IrregularOkurigana => "irregular okurigana",
            Information::OutdatedKanji => "outdated kanji",
            Information::OutdatedKana => "outdated kana",
            Information::Gikun => "gikun",
            Information::UsuallyKana => "usually written in kana",
        }
    }
}

impl ToSql<Integer, Pg> for Information {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <i32 as ToSql<Integer, Pg>>::to_sql(&(*self).into(), out)
    }
}

impl FromSql<Integer, Pg> for Information {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::try_from(<i32 as FromSql<Integer, Pg>>::from_sql(
            bytes,
        )?)?)
    }
}

impl TryFrom<i32> for Information {
    type Error = error::Error;
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        Ok(match i {
            0 => Self::Ateji,
            1 => Self::IrregularKana,
            2 => Self::IrregularKanji,
            3 => Self::IrregularOkurigana,
            4 => Self::OutdatedKanji,
            5 => Self::OutdatedKana,
            6 => Self::Gikun,
            7 => Self::UsuallyKana,
            _ => return Err(error::Error::ParseError),
        })
    }
}

impl Into<i32> for Information {
    fn into(self) -> i32 {
        match self {
            Self::Ateji => 0,
            Self::IrregularKana => 1,
            Self::IrregularKanji => 2,
            Self::IrregularOkurigana => 3,
            Self::OutdatedKanji => 4,
            Self::OutdatedKana => 5,
            Self::Gikun => 6,
            Self::UsuallyKana => 7,
        }
    }
}
