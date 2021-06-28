use deadpool_postgres::Pool;
use error::api_error::RestError;
use search::{query::Query, suggestions};

use super::{
    WordPair,
    storage::{KanjiMeaningSuggestionItem, K_MEANING_SUGGESTIONS},
    Response,
};

/// Returns kanji meaning suggestions
pub async fn suggestions(_client: &Pool, query: &Query) -> Result<Response, RestError> {
    let dict = match K_MEANING_SUGGESTIONS.get() {
        Some(v) => v,
        None => return Ok(Response::default()),
    };

    let mut items = match suggestions::kanji_meaning(dict, &query.query).await {
        Some(s) => s,
        None => return Ok(Response::default()),
    };

    items.dedup_by(|a, b| a.literal == b.literal);

    let res = items.into_iter().map(item_to_wp).take(10).collect();

    Ok(Response {
        suggestions: res,
        ..Default::default()
    })
}

fn item_to_wp(item: &KanjiMeaningSuggestionItem) -> WordPair {
    WordPair {
        primary: item.meaning.clone(),
        secondary: Some(item.literal.to_string()),
    }
}
