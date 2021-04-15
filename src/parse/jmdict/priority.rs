use std::{convert::TryFrom, io::Write};

use diesel::{
    deserialize,
    pg::Pg,
    serialize::{self, Output},
    sql_types::Text,
    types::{FromSql, ToSql},
};

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
        if value.starts_with("news") {
            (value.len() > 4).then(|| 0).ok_or(Error::Undefined)?;

            return Ok(Priority::News(value[4..5].parse()?));
        }

        if value.starts_with("ichi") {
            (value.len() > 4)
                .then(|| 0)
                .ok_or(error::Error::Undefined)?;

            return Ok(Priority::Ichi(value[4..5].parse()?));
        }

        if value.starts_with("spec") {
            (value.len() > 4)
                .then(|| 0)
                .ok_or(error::Error::Undefined)?;
            return Ok(Priority::Spec(value[4..5].parse()?));
        }

        if value.starts_with("gai") {
            (value.len() > 3)
                .then(|| 0)
                .ok_or(error::Error::Undefined)?;
            return Ok(Priority::Gai(value[3..4].parse()?));
        }

        if value.starts_with("nf") {
            (value.len() > 2)
                .then(|| 0)
                .ok_or(error::Error::Undefined)?;
            return Ok(Priority::Nf(value[2..].parse()?));
        }

        Err(Error::Undefined)
    }
}

impl ToSql<Text, Pg> for Priority {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let s: String = (*self).into();
        <&str as ToSql<Text, Pg>>::to_sql(&s.as_str(), out)
    }
}

impl FromSql<Text, Pg> for Priority {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        Ok(Self::try_from(
            (<String as FromSql<Text, Pg>>::from_sql(bytes)?).as_str(),
        )?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test() {
        let s = Priority::try_from("ichi1");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, Priority::Ichi(1));
        let p: String = s.into();
        assert_eq!(p, "ichi1");
    }

    #[test]
    fn test2() {
        let s = Priority::try_from("nf10");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, Priority::Nf(10));
        let p: String = s.into();
        assert_eq!(p, "nf10");
    }

    #[test]
    fn test3() {
        let s = Priority::try_from("nf4");
        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s, Priority::Nf(4));
        let p: String = s.into();
        assert_eq!(p, "nf4");
    }
}
