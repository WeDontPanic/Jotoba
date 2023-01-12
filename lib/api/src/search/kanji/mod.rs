use actix_web::web::{self, Data, Json};
use config::Config;
use types::{
    api::search::kanji::{Kanji, Response},
    jotoba::search::SearchTarget,
};

use super::{Result, SearchRequest};

/// Do a kanji search via API
pub async fn kanji_search(
    payload: Json<SearchRequest>,
    config: Data<Config>,
) -> Result<Json<Response>> {
    let query = super::parse_query(payload, SearchTarget::Kanji)?;
    let result = web::block(move || search::kanji::search(&query))
        .await??
        .items;
    Ok(Json(to_response(result, &config)))
}

#[inline]
fn to_response(items: Vec<search::kanji::result::Item>, config: &Config) -> Response {
    let kanji = items
        .into_iter()
        .map(|i| Kanji::from(&i.kanji, config.server.get_html_files()))
        .collect();
    Response { kanji }
}
