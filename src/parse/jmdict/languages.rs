use std::{io::Write, str::FromStr};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Text,
    types::{FromSql, ToSql},
};
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(
    AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Display,
)]
#[sql_type = "Text"]
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

impl ToSql<Text, Pg> for Language {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <&str as ToSql<Text, Pg>>::to_sql(&self.as_ref(), out)
    }
}

impl FromSql<Text, Pg> for Language {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::from_str(
            (<String as FromSql<Text, Pg>>::from_sql(bytes)?).as_str(),
        )?)
    }
}
