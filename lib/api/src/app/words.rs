use crate::app::new_page;

use super::{convert_payload, Result};
use actix_web::web::{self, Json};
use types::{
    api::app::{
        query::SearchPayload,
        responses::words::{self, Sentence},
    },
    jotoba::pagination::page::Page,
};

/// API response type
pub type Resp = Page<words::Response>;

/// Do an app word search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload).parse().unwrap();

    let result = web::block(move || search::word::search(&query)).await??;

    let (words, kanji) = result.get_items();
    let kanji = conv_kanji(&kanji);
    let words = words.into_iter().cloned().collect();

    let sentence = result
        .sentence_parts
        .map(|i| conv_sentence(i, result.sentence_index));
    let infl_info = result.inflection_info.map(|i| conv_infl_info(i));

    let original_query = result.searched_query;

    let res = words::Response::new(words, kanji, infl_info, sentence, original_query);
    let len = result.count as u32;

    let page = new_page(&payload, res, len, payload.settings.page_size);
    Ok(Json(page))
}

fn conv_kanji(
    kanji: &[&types::jotoba::kanji::Kanji],
) -> Vec<types::api::app::responses::kanji::Kanji> {
    kanji.iter().map(|i| (*i).clone().into()).collect()
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
