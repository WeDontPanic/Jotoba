use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Request struct for kanji_by_radicals endpoint
#[derive(Deserialize)]
pub struct RadicalsRequest {
    pub radicals: Vec<char>,
}

/// Response struct for kanji_by_radicals endpoint
#[derive(Serialize)]
pub struct RadicalsResponse {
    pub kanji: HashMap<i32, Vec<char>>,
    pub possible_radicals: Vec<char>,
}

/// Get kanji by its radicals
pub async fn kanji_by_radicals(
    payload: Json<RadicalsRequest>,
) -> Result<Json<RadicalsResponse>, actix_web::Error> {
    let kanji_retr = resources::get().kanji();

    let mut possible_radicals: HashSet<char> = HashSet::new();
    let mut kanji_res: HashMap<i32, Vec<char>> = HashMap::new();

    for kanji in kanji_retr.by_radicals(&payload.radicals) {
        kanji_res
            .entry(kanji.stroke_count as i32)
            .or_default()
            .push(kanji.literal);

        if let Some(parts) = &kanji.parts {
            possible_radicals.extend(parts);
        }
    }

    let possible_radicals = possible_radicals.into_iter().collect();

    Ok(Json(RadicalsResponse {
        possible_radicals,
        kanji: kanji_res,
    }))
}
