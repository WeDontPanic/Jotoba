use super::new_page;

use super::convert_payload;
use crate::app::Result;
use actix_web::web::{self, Json};
use error::api_error::RestError;
use types::{
    api::app::search::{
        query::SearchPayload,
        responses::{sentences, Response},
    },
    jotoba::search::QueryType,
};

/// API response type
pub type Resp = Response<sentences::Response>;

/// Do an app sentence search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;

    let query_c = query.clone();
    let result = web::block(move || search::sentence::search(&query_c)).await??;

    let items = result
        .items
        .into_iter()
        .map(|i| convert_sentence(i.sentence))
        .collect::<Vec<_>>();

    let res = sentences::Response::new(items);
    let len = result.len as u32;

    let page = new_page(&payload, res, len, payload.settings.page_size);
    let res = super::new_response(page, QueryType::Sentences, &query);
    Ok(Json(res))
}

#[inline]
pub(crate) fn convert_sentence(
    sentence: search::sentence::result::Sentence,
) -> sentences::Sentence {
    sentences::Sentence::new(sentence.id, sentence.furigana, sentence.translation)
}
