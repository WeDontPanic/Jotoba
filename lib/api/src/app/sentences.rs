use crate::app::new_page;

use super::{convert_payload, Result};
use actix_web::web::{self, Json};
use error::api_error::RestError;
use types::{
    api::app::{query::SearchPayload, responses::sentences},
    jotoba::pagination::page::Page,
};

/// API response type
pub type Resp = Page<sentences::Response>;

/// Do an app sentence search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;

    let result = web::block(move || search::sentence::search(&query)).await??;

    let items = result
        .items
        .into_iter()
        .map(|i| convert_sentence(i.sentence))
        .collect::<Vec<_>>();

    let res = sentences::Response::new(items);
    let len = result.len as u32;

    let page = new_page(&payload, res, len, payload.settings.page_size);

    Ok(Json(page))
}

#[inline]
fn convert_sentence(sentence: search::sentence::result::Sentence) -> sentences::Sentence {
    sentences::Sentence::new(sentence.furigana, sentence.translation)
}
