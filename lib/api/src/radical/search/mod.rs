mod jp_search;
mod meaning;

use std::{
    collections::{BTreeSet, HashMap},
    str::FromStr,
};

use actix_web::{web::Json, HttpRequest};
use error::api_error::RestError;
use japanese::JapaneseExt;
use types::{
    api::radical::search::{Request, Response},
    jotoba::languages::Language,
};

/// Search for radicals
pub async fn search_radical(
    mut payload: Json<Request>,
    request: HttpRequest,
) -> Result<Json<Response>, actix_web::Error> {
    verify_payload(&mut payload)?;

    let rad_res;
    let mut kanji_res = vec![];

    if payload.query.is_japanese() {
        rad_res = jp_search::search(&payload.query);
        kanji_res = jp_search::similar_kanji_search(&payload.query);
    } else {
        rad_res = meaning::search(&payload.query, user_lang(&request));
    }

    if rad_res.is_empty() && kanji_res.is_empty() {
        return Ok(Json(Response::default()));
    }

    let radicals = map_radicals(&rad_res);

    Ok(Json(Response {
        radicals,
        kanji: kanji_res,
    }))
}

/// Load the users language from cookies
fn user_lang(request: &HttpRequest) -> Language {
    request
        .cookie("default_lang")
        .and_then(|i| Language::from_str(i.value()).ok())
        .unwrap_or_default()
}

/// Maps radicals by its literals to ResRadical with its stroke count
fn map_radicals(inp: &[char]) -> HashMap<u8, BTreeSet<char>> {
    let mut radicals: HashMap<u8, BTreeSet<char>> = HashMap::with_capacity(inp.len());

    for (lit, strokes) in inp
        .iter()
        .filter_map(|lit| japanese::radicals::get_radical(*lit))
    {
        radicals.entry(strokes as u8).or_default().insert(lit);
    }

    radicals
}

/// Verifies the payload itself and returns a proper error if the request is invalid
fn verify_payload(payload: &mut Request) -> Result<(), RestError> {
    if payload.query.trim().is_empty() {
        return Err(RestError::BadRequest);
    }

    payload.query = payload.query.trim().to_string();
    Ok(())
}
