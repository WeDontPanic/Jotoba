pub mod meaning;
pub mod reading;

use error::api_error::RestError;
use search::query::{Query, QueryLang};
use types::api::completions::Response;

/// Returns kanji suggestions
pub(crate) async fn suggestions(query: Query) -> Result<Response, RestError> {
    if query.language == QueryLang::Foreign {
        meaning::suggestions(&query).await
    } else {
        Ok(Response::default())
    }
}
