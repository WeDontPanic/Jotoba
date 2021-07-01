pub mod api_error;

use std::{fmt::Display, num::ParseIntError, string::FromUtf8Error};

use deadpool_postgres::PoolError;
use strum::ParseError;

#[derive(Debug)]
pub enum Error {
    NotFound,
    ParseInt(ParseIntError),
    Utf8Error(FromUtf8Error),
    Utf8StrError(std::str::Utf8Error),
    ParseError,
    Undefined,
    IoError(std::io::Error),
    Checkout(r2d2::Error),
    PoolError(PoolError),
    Postgres(tokio_postgres::Error), // new db error
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::Postgres(err)
    }
}

impl From<PoolError> for Error {
    fn from(err: PoolError) -> Self {
        Self::PoolError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8Error(err)
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::VariantNotFound => Self::ParseError,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self::Utf8StrError(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
