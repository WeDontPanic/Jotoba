use deadpool_postgres::Pool;
use error::api_error::RestError;
use search::query::Query;

use super::{
    storage::{NAME_NATIVE, NAME_TRANSCRIPTIONS},
    Response, WordPair,
};

/// Returns trascripted name suggestions based on the input query
pub async fn transcription_suggestions(
    _client: &Pool,
    query: &Query,
) -> Result<Response, RestError> {
    let dict = match NAME_TRANSCRIPTIONS.get() {
        Some(v) => v,
        None => return Ok(Response::default()),
    };

    let mut items = search::suggestions::generic(&dict, &query.query).await;

    items.dedup_by(|a, b| a.name == b.name);

    let res: Vec<WordPair> = items.into_iter().map(|i| i.into()).take(10).collect();

    Ok(Response {
        suggestions: res,
        ..Default::default()
    })
}

/// Returns native name suggestions based on the input query
pub async fn native_suggestions(_client: &Pool, query: &Query) -> Result<Response, RestError> {
    let query_str = &query.query;

    let dict = match NAME_NATIVE.get() {
        Some(v) => v,
        None => return Ok(Response::default()),
    };

    let mut items = search::suggestions::japanese(&dict, query_str).await;

    items.dedup_by(|a, b| a.name == b.name);

    let res: Vec<WordPair> = items.into_iter().map(|i| i.into()).take(10).collect();

    Ok(Response {
        suggestions: res,
        ..Default::default()
    })
}
