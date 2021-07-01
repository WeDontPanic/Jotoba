use actix_web::web::{Data, Json};
use deadpool_postgres::Pool;
use search::query_parser::QueryType::Kanji;

use self::response::Response;

use super::{Result, SearchRequest};

pub mod response;

/// Do a kanji search via API
pub async fn kanji_search(
    payload: Json<SearchRequest>,
    pool: Data<Pool>,
) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, Kanji)?;

    Ok(Json(search::kanji::search(&pool, &query).await?.into()))
}
