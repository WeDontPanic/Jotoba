use actix_web::web::{self, Json};
use search::query_parser::QueryType::Kanji;

use self::response::Response;

use super::{Result, SearchRequest};

pub mod response;

/// Do a kanji search via API
pub async fn kanji_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, Kanji)?;
    let result = web::block(move || search::kanji::search(&query))
        .await??
        .items;
    Ok(Json(result.into()))
}
