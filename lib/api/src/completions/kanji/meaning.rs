use error::api_error::{Origin, RestError};
use search::{query::Query, suggestions};

use types::api::completions::WordPair;

use super::super::{storage::K_MEANING_SUGGESTIONS, Response};

/// Returns kanji meaning suggestions
pub async fn suggestions(query: &Query) -> Result<Response, RestError> {
    let dict = K_MEANING_SUGGESTIONS
        .get()
        .ok_or(RestError::Missing(Origin::Suggestions))?;

    let mut items = suggestions::kanji_meaning(dict, &query.query).await;

    items.dedup_by(|a, b| a.literal == b.literal);

    let res: Vec<WordPair> = items.into_iter().map(|i| i.into()).take(10).collect();

    Ok(Response {
        suggestions: res,
        ..Default::default()
    })
}
