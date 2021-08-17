use actix_web::web::Json;
use search::query_parser::QueryType::Kanji;

use self::response::Response;

use super::{Result, SearchRequest};

pub mod response;

/// Do a kanji search via API
pub async fn kanji_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = SearchRequest::parse(payload, Kanji)?;

    // TODO: implement
    unimplemented!()
    //Ok(Json(search::kanji::search(&pool, &query).await?.into()))
}
