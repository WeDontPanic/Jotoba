use super::{Result, SearchRequest};
use actix_web::web::{self, Json};
use types::{api::search::word::Response, jotoba::search::QueryType};

/// Do a word search via API
pub async fn word_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = super::parse_query(payload, QueryType::Words)?;
    let result = web::block(move || search::word::search(&query)).await??;
    let response: Response = result.get_items().into();
    Ok(Json(response))
}
