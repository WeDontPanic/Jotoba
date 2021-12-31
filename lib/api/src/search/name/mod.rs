use actix_web::web::{self, Json};
use types::{api::search::name::Response, jotoba::search::QueryType};

use super::{Result, SearchRequest};

/// Do a name search via API
pub async fn name_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = super::parse_query(payload, QueryType::Kanji)?;
    let result = web::block(move || search::name::search(&query)).await??;
    Ok(Json(result.items.into()))
}
