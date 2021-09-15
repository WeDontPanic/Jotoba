use actix_web::web::Json;
use itertools::Itertools;
use search::query_parser::QueryType::Kanji;

use self::response::Response;

use super::{Result, SearchRequest};

pub mod response;

/// Do a Sentence search via API
pub async fn sentence_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, Kanji)?;

    Ok(Json(
        search::sentence::search(&query)
            .await?
            .0
            .into_iter()
            .map(|i| i.sentence)
            .collect_vec()
            .into(),
    ))
}
