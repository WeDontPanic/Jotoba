use actix_web::web::{self, Json};
use types::{
    api::search::kanji::{Kanji, Response},
    jotoba::search::QueryType,
};

use super::{Result, SearchRequest};

/// Do a kanji search via API
pub async fn kanji_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = super::parse_query(payload, QueryType::Kanji)?;
    let result = web::block(move || search::kanji::search(&query))
        .await??
        .items;
    Ok(Json(to_response(result)))
}

#[inline]
fn to_response(items: Vec<search::kanji::result::Item>) -> Response {
    let kanji = items.into_iter().map(|i| Kanji::from(&i.kanji)).collect();
    Response { kanji }
}
