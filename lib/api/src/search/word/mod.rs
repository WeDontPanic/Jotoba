pub mod response;

use self::response::Response;

use super::{Result, SearchRequest};

use actix_web::web::{Data, Json};
use deadpool_postgres::Pool;
use search::query_parser::QueryType::Words;

/// Do a word search via API
pub async fn word_search(payload: Json<SearchRequest>, pool: Data<Pool>) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, Words)?;

    Ok(Json(search::word::search(&pool, &query).await?.into()))
}
