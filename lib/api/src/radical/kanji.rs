use actix_web::web::Json;
use std::collections::{HashMap, HashSet};
use types::api::radical::find_kanji::{Request, Response};

/// Get kanji by its radicals
pub async fn kanji_by_radicals(
    payload: Json<Request>,
) -> Result<Json<Response>, actix_web::Error> {
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

    Ok(Json(Response {
        possible_radicals,
        kanji: kanji_res,
    }))
}
