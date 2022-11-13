use super::new_page;

use super::convert_payload;
use crate::app::Result;
use actix_web::web::{self, Json};
use error::api_error::RestError;

use types::{
    api::app::search::{
        query::SearchPayload,
        responses::{
            k_compounds::{CompoundResponse, CompoundSet, CompoundWord},
            kanji, Response,
        },
    },
    jotoba::{
        languages::Language,
        search::SearchTarget,
        words::{filter_languages, Word},
    },
};

/// API response type
pub type SearchResp = Response<kanji::KanjiResponse>;

/// Do an app kanji search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<SearchResp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;

    let query_c = query.clone();
    let result = web::block(move || search::kanji::search(&query_c)).await??;

    let items = result
        .items
        .into_iter()
        .map(|i| {
            let k: kanji::Kanji = i.kanji.into();
            k
        })
        .collect::<Vec<_>>();

    let len = result.total_len as u32;
    let kanji = kanji::KanjiResponse::new(items);
    let page = new_page(&payload, kanji, len, payload.settings.kanji_page_size);
    Ok(Json(super::new_response(page, SearchTarget::Kanji, &query)))
}

/// Kanji compound request
pub async fn reading_compounds(payload: Json<SearchPayload>) -> Result<Json<CompoundResponse>> {
    let lang = payload.settings.user_lang;
    let show_english = payload.settings.show_english;

    let compounds: Vec<_> = payload
        .query_str
        .chars()
        .filter_map(|i| resources::get().kanji().by_literal(i))
        .map(|i| {
            let on_words = convert_dicts(&i.on_dicts, lang, show_english);
            let kun_words = convert_dicts(&i.on_dicts, lang, show_english);
            CompoundSet::new(on_words, kun_words)
        })
        .collect();
    Ok(Json(CompoundResponse::new(compounds)))
}

#[inline]
fn convert_dicts(dicts: &Vec<u32>, lang: Language, show_english: bool) -> Vec<CompoundWord> {
    load_dicts(dicts, lang, show_english)
        .into_iter()
        .filter_map(|j| Some(CompoundWord::from_word(&j)))
        .collect::<Vec<_>>()
}

#[inline]
fn load_dicts(dicts: &Vec<u32>, lang: Language, show_english: bool) -> Vec<Word> {
    let word_storage = resources::get().words();
    let mut words: Vec<_> = dicts
        .iter()
        .filter_map(|j| word_storage.by_sequence(*j))
        .cloned()
        .collect();
    filter_languages(words.iter_mut(), lang, show_english);
    words
}
