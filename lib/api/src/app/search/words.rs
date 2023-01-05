use super::new_page;

use super::convert_payload;
use crate::app::Result;
use actix_web::web::{self, Json};
use error::api_error::RestError;
use search::word::Search;
use search::SearchExecutor;
use types::{
    api::app::search::{
        query::SearchPayload,
        responses::{
            words::{self, Sentence},
            Response,
        },
    },
    jotoba::search::SearchTarget,
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
    let result = web::block(move || {
        let search = Search::new(&query_c);
        SearchExecutor::new(search).run()
    })
    .await?;

    let kanji = search::word::kanji::load_word_kanji_info(&result.items)
        .into_iter()
        .map(|i| i.into())
        .collect::<Vec<_>>();

    let words = result
        .items
        .iter()
        .map(|i| super::super::conv_word(i.clone(), user_lang))
        .collect::<Vec<_>>();

    let s_index = result.sentence_index();

    let number = result.number.clone();

    let sentence = result
        .other_data
        .sentence
        .and_then(|i| i.parts)
        .map(|i| conv_sentence(i, s_index));
    let infl_info = result.other_data.inflection.map(|i| conv_infl_info(i));

    let original_query = result.other_data.raw_query.clone();

    let res = words::Response::new(words, kanji, infl_info, sentence, original_query, number);
    let len = result.total as u32;

    let page = new_page(&payload, res, len, payload.settings.page_size);
    let res = super::new_response(page, SearchTarget::Words, &query);
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
