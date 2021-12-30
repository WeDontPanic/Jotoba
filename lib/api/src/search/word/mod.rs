pub mod response;

use self::response::Response;

use super::{Result, SearchRequest};

use actix_web::web::{self, Json};
use types::jotoba::search::QueryType;

/// Do a word search via API
pub async fn word_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, QueryType::Words)?;

    let result = web::block(move || search::word::search(&query)).await??;

    Ok(Json(result.into()))
}
