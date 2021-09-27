use actix_web::web::{self, Json};
use search::query_parser::QueryType::Kanji;

use self::response::Response;

use super::{Result, SearchRequest};

pub mod response;

/// Do a Sentence search via API
pub async fn sentence_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, Kanji)?;

    let result = web::block(move || search::sentence::search(&query)).await??;

    Ok(Json(
        result
            .items
            .into_iter()
            .map(|i| i.sentence)
            .collect::<Vec<_>>()
            .into(),
    ))
}
