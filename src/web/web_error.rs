use actix_web::{http::StatusCode, HttpResponse, ResponseError};

use crate::templates;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("internal")]
    Internal,
    #[error("not found")]
    NotFound,
}

/// Informatin to print on the error page
pub struct InfoText {
    pub primary: &'static str,
    pub secondary: &'static str,
}

// Treat all crate::error::Error as Internal error
impl From<crate::error::Error> for Error {
    fn from(_: crate::error::Error) -> Self {
        Self::Internal
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        // Render the error template
        HttpResponse::Ok().body(render!(
            templates::error_page,
            self.status_code().as_u16(),
            self.get_info_text()
        ))
    }
}

impl Error {
    /// Return an [`InfoText`] based on the error suitable for displaying on the error site
    fn get_info_text(&self) -> InfoText {
        let (primary, secondary) = {
            match self {
                Error::Internal => ("Sorry", "try again later"),
                Error::NotFound => ("The page", "was not found"),
            }
        };

        InfoText { primary, secondary }
    }
}

/// Not found error handler
pub async fn not_found() -> Result<HttpResponse, Error> {
    Err(Error::NotFound)
}
