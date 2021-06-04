use std::{fmt::Display, io::Write, str::FromStr};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Text,
    types::{FromSql, ToSql},
};

use localization::traits::Translatable;
use strum_macros::{AsRefStr, EnumString};

#[derive(AsExpression, FromSqlRow, Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
#[sql_type = "Text"]
pub enum Dialect {
    #[strum(serialize = "hob")]
    Hokkaido,
    #[strum(serialize = "ksb")]
    Kansai,
    #[strum(serialize = "ktb")]
    Kantou,
    #[strum(serialize = "kyb")]
    Kyoto,
    #[strum(serialize = "kyu")]
    Kyuushuu,
    #[strum(serialize = "nab")]
    Nagano,
    #[strum(serialize = "osb")]
    Osaka,
    #[strum(serialize = "rkb")]
    Ryuukyuu,
    #[strum(serialize = "thb")]
    Touhoku,
    #[strum(serialize = "tsb")]
    Tosa,
    #[strum(serialize = "tsug")]
    Tsugaru,
}

impl Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<&'static str> for Dialect {
    fn into(self) -> &'static str {
        match self {
            Dialect::Hokkaido => "Hokkaido",
            Dialect::Kansai => "Kansai",
            Dialect::Kantou => "Kantou",
            Dialect::Kyoto => "Kyoto",
            Dialect::Kyuushuu => "Kyuushuu",
            Dialect::Nagano => "Nagano",
            Dialect::Osaka => "Osaka",
            Dialect::Ryuukyuu => "Ryuukyuu",
            Dialect::Touhoku => "Touhoku",
            Dialect::Tosa => "Tosa",
            Dialect::Tsugaru => "Tsugaru",
        }
    }
}

impl Translatable for Dialect {
    fn get_id(&self) -> &'static str {
        "{} dialect"
    }
}

impl ToSql<Text, Pg> for Dialect {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <&str as ToSql<Text, Pg>>::to_sql(&self.as_ref(), out)
    }
}

impl FromSql<Text, Pg> for Dialect {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::from_str(
            (<String as FromSql<Text, Pg>>::from_sql(bytes)?).as_str(),
        )?)
    }
}
