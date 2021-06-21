use std::convert::TryFrom;

use postgres_types::{accepts, to_sql_checked};
use tokio_postgres::types::{FromSql, ToSql};

use crate::error::{self, Error};

/// Priority indicator of kanji/reading element
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Priority {
    News(u8),
    Ichi(u8),
    Spec(u8),
    Gai(u8),
    Nf(u8),
}

impl Into<String> for Priority {
    fn into(self) -> String {
        match self {
            Priority::News(v) => format!("news{}", v),
            Priority::Ichi(v) => format!("ichi{}", v),
            Priority::Spec(v) => format!("spec{}", v),
            Priority::Gai(v) => format!("gai{}", v),
            Priority::Nf(v) => format!("nf{}", v),
        }
    }
}

impl TryFrom<&str> for Priority {
    type Error = error::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(end) = value.strip_prefix("news") {
            return Ok(Priority::News(end.parse()?));
        }

        if let Some(end) = value.strip_prefix("ichi") {
            return Ok(Priority::Ichi(end.parse()?));
        }

        if let Some(end) = value.strip_prefix("spec") {
            return Ok(Priority::Spec(end.parse()?));
        }

        if let Some(end) = value.strip_prefix("gai") {
            return Ok(Priority::Gai(end.parse()?));
        }

        if let Some(end) = value.strip_prefix("nf") {
            return Ok(Priority::Nf(end.parse()?));
        }

        Err(Error::Undefined)
    }
}

impl<'a> FromSql<'a> for Priority {
    fn from_sql(
        ty: &tokio_postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        Ok(Self::try_from(
            <String as tokio_postgres::types::FromSql>::from_sql(ty, raw)?.as_str(),
        )?)
    }

    accepts!(TEXT);
}

impl ToSql for Priority {
    fn to_sql(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let i: String = (*self).into();
        Ok(<String as ToSql>::to_sql(&i, ty, out)?)
    }

    accepts!(TEXT);

    to_sql_checked!();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_priority_ichi() {
        let s = Priority::try_from("ichi1");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, Priority::Ichi(1));
        let p: String = s.into();
        assert_eq!(p, "ichi1");
        let s = Priority::try_from("ichi");
        assert!(s.is_err());
    }

    #[test]
    fn test_priority_nf() {
        let s = Priority::try_from("nf10");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, Priority::Nf(10));
        let p: String = s.into();
        assert_eq!(p, "nf10");
        let s = Priority::try_from("nf4");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, Priority::Nf(4));
        let p: String = s.into();
        assert_eq!(p, "nf4");

        let s = Priority::try_from("nf");
        assert!(s.is_err());
    }

    #[test]
    fn test_priority_news() {
        let s = Priority::try_from("news10");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, Priority::News(10));
        let p: String = s.into();
        assert_eq!(p, "news10");

        let s = Priority::try_from("news");
        assert!(s.is_err());
    }
}
