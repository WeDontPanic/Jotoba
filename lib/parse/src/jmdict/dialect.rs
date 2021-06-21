use std::{fmt::Display, io::Write, str::FromStr};

use localization::{language::Language, traits::Translatable, TranslationDict};
use postgres_types::{accepts, to_sql_checked};
use strum_macros::{AsRefStr, EnumString};
use tokio_postgres::types::{FromSql, ToSql};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString)]
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

    fn gettext_custom(&self, dict: &TranslationDict, language: Option<Language>) -> String {
        dict.gettext_fmt("{} dialect", &[self.gettext(dict, language)], language)
    }
}

impl<'a> FromSql<'a> for Dialect {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self::from_str(
            <String as tokio_postgres::types::FromSql>::from_sql(ty, raw)?.as_str(),
        )?)
    }

    accepts!(TEXT);
}

impl ToSql for Dialect {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        Ok(<&str as ToSql>::to_sql(&self.as_ref(), ty, out)?)
    }

    accepts!(TEXT);

    to_sql_checked!();
}
