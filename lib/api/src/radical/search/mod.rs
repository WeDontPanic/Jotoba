mod jp_search;
mod meaning;

use std::collections::{HashMap, HashSet};

use actix_web::web::Json;
use error::api_error::RestError;
use japanese::JapaneseExt;
use serde::{Deserialize, Serialize};

/// Request struct for kanji_by_radicals endpoint
#[derive(Deserialize)]
pub struct RadSearchRequest {
    pub query: String,
    #[serde(default)]
    pub picked_radicals: Vec<char>,
}

/// Response struct for kanji_by_radicals endpoint
#[derive(Serialize, Default)]
pub struct RadSearchResponse {
    pub radicals: HashMap<u8, HashSet<ResRadical>>,
}

/// Single radical with its enabled/disabled state, representing whether it can be used together
/// with the currently picked radicals or not.
#[derive(Serialize, Hash, Eq, PartialEq)]
pub struct ResRadical {
    #[serde(rename = "l")]
    literal: char,
    #[serde(rename = "p")]
    possible: bool,
}

/// Search for radicals
pub async fn search_radical(
    mut payload: Json<RadSearchRequest>,
) -> Result<Json<RadSearchResponse>, actix_web::Error> {
    verify_payload(&mut payload)?;

    let res = if !payload.query.is_japanese() {
        meaning::search(&payload.query)
    } else {
        jp_search::search(&payload.query)
    };

    if res.is_empty() {
        return Ok(Json(RadSearchResponse::default()));
    }

    let mut radicals = HashMap::with_capacity(res.len());

    let possible_radicals = get_possible_radicals(&payload.picked_radicals);

    for (rad, strokes) in map_radicals(&res, possible_radicals) {
        radicals
            .entry(strokes as u8)
            .or_insert_with(|| HashSet::new())
            .insert(rad);
    }

    Ok(Json(RadSearchResponse { radicals }))
}

/// Maps radicals by its literals to ResRadical with its stroke count
fn map_radicals(inp: &[char], possible_radicals: Option<HashSet<char>>) -> Vec<(ResRadical, i32)> {
    let mut res = Vec::with_capacity(inp.len());

    for (lit, strokes) in inp
        .iter()
        .filter_map(|lit| japanese::radicals::get_radical(*lit))
    {
        let possible = possible_radicals
            .as_ref()
            .map(|i| i.contains(&lit))
            .unwrap_or(true);

        let res_rad = ResRadical {
            literal: lit,
            possible,
        };

        res.push((res_rad, strokes));
    }

    res
}

/// Returns a HashSet of radicals that are still possible. Returns `None` if no radicals were provided.
fn get_possible_radicals(radicals: &[char]) -> Option<HashSet<char>> {
    if radicals.is_empty() {
        return None;
    }

    let mut possible_radicals: HashSet<char> = HashSet::new();

    for kanji in resources::get().kanji().by_radicals(radicals) {
        if let Some(parts) = &kanji.parts {
            possible_radicals.extend(parts);
        }
    }

    Some(possible_radicals)
}

/// Verifies the payload itself and returns a proper error if the request is invalid
fn verify_payload(payload: &mut RadSearchRequest) -> Result<(), RestError> {
    if payload.query.trim().is_empty() {
        return Err(RestError::BadRequest);
    }

    payload.query = payload.query.trim().to_string();
    Ok(())
}
