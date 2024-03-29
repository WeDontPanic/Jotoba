use actix_web::web::Json;
use intmap::{int_set::IntSet, IntMap};
use std::{collections::HashMap, time::Instant};
use types::api::app::radical::find_kanji::{Request, Response};

/// Get kanji by its radicals
pub async fn kanji_by_radicals(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    let start = Instant::now();
    let res = find_kanji(&payload.radicals);
    log::debug!("Radical results took: {:?}", start.elapsed());

    Ok(Json(res))
}

pub fn find_kanji(rads: &[char]) -> Response {
    let mut possible_rads_set = IntSet::with_capacity(rads.len() * 3);
    let mut kanji_res: IntMap<Vec<char>> = IntMap::with_capacity(8);

    let k_retrieve = resources::get().kanji();
    for kanji in k_retrieve.by_radicals(rads) {
        push_or_insert(&mut kanji_res, kanji.stroke_count as u32, kanji.literal);

        if !kanji.parts.is_empty() {
            possible_rads_set.reserve(kanji.parts.len());
            possible_rads_set.extend(kanji.parts.iter().map(|i| *i as u32));
        }
    }

    let mut possible_rads = HashMap::<u32, Vec<char>>::new();
    for i in possible_rads_set {
        let c = unsafe { char::from_u32_unchecked(i) };
        let s_count = japanese::radicals::get_stroke_count(c).unwrap();
        possible_rads.entry(s_count as u32).or_default().push(c);
    }

    // Sort all radicals
    for (_, v) in possible_rads.iter_mut() {
        v.sort_unstable();
    }

    let mut kanji_res2 = HashMap::<u32, Vec<char>>::with_capacity(kanji_res.len());
    kanji_res2.extend(kanji_res);

    Response {
        possible_radicals: possible_rads,
        kanji: kanji_res2,
    }
}

fn push_or_insert<T>(map: &mut IntMap<Vec<T>>, key: u32, item: T) {
    if let Some(s) = map.get_mut(key) {
        s.push(item);
        return;
    }

    let capacity = (25u32.saturating_sub(key) + 1) * 2;
    let mut new_vec = Vec::with_capacity(capacity as usize);
    new_vec.push(item);
    map.insert(key, new_vec);
}
