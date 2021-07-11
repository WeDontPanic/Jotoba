#![allow(dead_code, unreachable_patterns)]

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq)]
pub enum Origin {
    Radicals,
    Suggestions,
}

impl std::fmt::Debug for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Origin::Radicals => "radicals",
                Origin::Suggestions => "suggestions",
            }
        )
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq)]
pub enum RestError {
    #[error("Not found")]
    NotFound,

    #[error("Bad request")]
    BadRequest,

    #[error("Internal server error")]
    Internal,

    #[error("Timeout exceeded")]
    Timeout,

    #[error("missing {0:?}")]
    Missing(Origin),
}

/// Error response format. Used as json encoding structure
#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl RestError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => "NotFound".to_string(),
            Self::BadRequest => "BadRequest".to_string(),
            Self::Internal => "InternalError".to_string(),
            Self::Timeout => "Timeout".to_string(),
            _ => "InternalError".to_string(),
        }
    }
}

/// Implement ResponseError trait. Required for actix web
impl ResponseError for RestError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Timeout => StatusCode::REQUEST_TIMEOUT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

impl From<super::Error> for RestError {
    fn from(err: super::Error) -> Self {
        match err {
            crate::Error::NotFound => Self::NotFound,
            crate::Error::PoolError(pool_err) => pool_err.into(),
            _ => Self::Internal,
        }
    }
}

impl From<PoolError> for RestError {
    fn from(e: PoolError) -> Self {
        match e {
            _ => Self::Internal,
        }
    }
}

impl From<tokio_postgres::Error> for RestError {
    fn from(e: tokio_postgres::Error) -> Self {
        Self::Internal
    }
}
