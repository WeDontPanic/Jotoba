pub mod meaning;
pub mod reading;

use error::api_error::RestError;
use search::query::{Query, QueryLang};
use types::api::completions::Response;

/// Returns kanji suggestions
pub(crate) fn suggestions(query: Query) -> Result<Response, RestError> {
    Ok(match query.language {
        QueryLang::Foreign => meaning::suggestions(&query)?,
        QueryLang::Japanese => japanese_suggestions(&query)?,
        /*
        QueryLang::Korean => todo!(),
        QueryLang::Undetected => todo!(),
        */
        _ => Response::default(),
    })
}

fn japanese_suggestions(query: &Query) -> Result<Response, RestError> {
    let mut suggestions =
        super::words::native::suggestions(&query, &[]).ok_or(RestError::Internal)?;

    // romev entries without kanji
    suggestions.retain(|i| i.secondary.is_some());

    Ok(Response {
        suggestions,
        ..Default::default()
    })
}
