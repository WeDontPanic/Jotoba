use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};

#[cfg(not(feature = "sentry_error"))]
use log::error;

use crate::templates;

#[derive(Debug)]
pub enum Error {
    Internal,
    NotFound,
    SearchTimeout,
    BadRequest,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Informatin to print on the error page
pub struct InfoText {
    pub primary: &'static str,
    pub secondary: &'static str,
}

// Treat all crate::error::Error as Internal error
impl From<error::Error> for Error {
    fn from(err: error::Error) -> Self {
        #[cfg(feature = "sentry_error")]
        sentry::capture_error(&err);

        #[cfg(not(feature = "sentry_error"))]
        error!("{}", err);

        Self::Internal
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::SearchTimeout => StatusCode::REQUEST_TIMEOUT,
            Error::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Render the error template
        HttpResponse::Ok().body(
            render!(
                templates::error_page,
                self.status_code().as_u16(),
                self.get_info_text()
            )
            .render(),
        )
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::BadRequest
    }
}

impl From<BlockingError> for Error {
    #[inline]
    fn from(_: BlockingError) -> Self {
        Self::Internal
    }
}

impl Error {
    /// Return an [`InfoText`] based on the error suitable for displaying on the error site
    fn get_info_text(&self) -> InfoText {
        let (primary, secondary) = {
            match self {
                Error::Internal => ("Sorry", "try again later"),
                Error::NotFound => ("The page", "was not found"),
                Error::SearchTimeout => ("Search", "timed out"),
                Error::BadRequest => ("Bad request", ""),
            }
        };

        InfoText { primary, secondary }
    }
}

/// Not found error handler
pub async fn not_found() -> Result<HttpResponse, Error> {
    Err(Error::NotFound)
}
