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
    api::radical::search::{Request, ResRadical, Response},
    jotoba::languages::Language,
};

/// Search for radicals
pub async fn search_radical(
    mut payload: Json<Request>,
    request: HttpRequest,
) -> Result<Json<Response>, actix_web::Error> {
    verify_payload(&mut payload)?;

    let user_lang = request
        .cookie("default_lang")
        .and_then(|i| Language::from_str(i.value()).ok())
        .unwrap_or_default();

    let res = if !payload.query.is_japanese() {
        meaning::search(&payload.query, user_lang)
    } else {
        jp_search::search(&payload.query)
    };

    if res.is_empty() {
        return Ok(Json(Response::default()));
    }

    Ok(Json(Response {
        radicals: map_radicals(&res),
    }))
}

/// Maps radicals by its literals to ResRadical with its stroke count
fn map_radicals(inp: &[char]) -> HashMap<u8, BTreeSet<ResRadical>> {
    let mut radicals = HashMap::with_capacity(inp.len());

    for (lit, strokes) in inp
        .iter()
        .filter_map(|lit| japanese::radicals::get_radical(*lit))
    {
        radicals
            .entry(strokes as u8)
            .or_insert_with(|| BTreeSet::new())
            .insert(ResRadical { literal: lit });
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
