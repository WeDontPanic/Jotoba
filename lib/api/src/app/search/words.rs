use super::new_page;

use super::convert_payload;
use crate::app::Result;
use actix_web::web::{self, Json};
use error::api_error::RestError;
use types::{
    api::app::search::{
        query::SearchPayload,
        responses::{
            words::{self, Sentence},
            Response,
        },
    },
    jotoba::search::QueryType,
};

/// API response type
pub type Resp = Response<words::Response>;

/// Do an app word search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;
    let user_lang = query.settings.user_lang;

    let query_c = query.clone();
    let result = web::block(move || search::word::search(&query_c)).await??;

    let words = result
        .words()
        .map(|i| super::super::conv_word(i.clone(), user_lang))
        .collect();

    let sentence = result
        .sentence_parts
        .map(|i| conv_sentence(i, result.sentence_index));
    let infl_info = result.inflection_info.map(|i| conv_infl_info(i));

    let original_query = result.searched_query;

    let res = words::Response::new(words, infl_info, sentence, original_query);
    let len = result.count as u32;

    let page = new_page(&payload, res, len, payload.settings.page_size);
    let res = super::new_response(page, QueryType::Words, &query);
    Ok(Json(res))
}

fn conv_sentence(sentence: sentence_reader::Sentence, index: usize) -> Sentence {
    let parts = sentence
        .into_parts()
        .into_iter()
        .map(|i| i.into())
        .collect();
    Sentence::new(index, parts)
}

fn conv_infl_info(infl_info: search::word::result::InflectionInformation) -> words::InflectionInfo {
    words::InflectionInfo::new(infl_info.inflections, infl_info.lexeme)
}
