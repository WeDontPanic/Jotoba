use actix_web::web::{Data, Json};
use deadpool_postgres::Pool;
use itertools::Itertools;
use search::query_parser::QueryType::Kanji;

use self::response::Response;

use super::{Result, SearchRequest};

pub mod response;

/// Do a Sentence search via API
pub async fn sentence_search(
    payload: Json<SearchRequest>,
    pool: Data<Pool>,
) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, Kanji)?;

    Ok(Json(
        search::sentence::search(&pool, &query)
            .await?
            .into_iter()
            .map(|i| i.sentence)
            .collect_vec()
            .into(),
    ))
}
