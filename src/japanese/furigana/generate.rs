use std::sync::Mutex;

use super::{
    super::{text_parts, JapaneseExt},
    calc_kanji_readings, from_str,
};
use crate::{cache::SharedCache, utils::real_string_len, DbConnection};
use diesel::prelude::*;
use itertools::Itertools;
use once_cell::sync::Lazy;

/// An in memory Cache for kanji items
static KANJICACHE: Lazy<Mutex<SharedCache<String, (Option<Vec<String>>, Option<Vec<String>>)>>> =
    Lazy::new(|| Mutex::new(SharedCache::with_capacity(10000)));

/// Encodes furigana readings of a japanese word/sentence and returns it in form of a string which
/// can be parsed later on
pub fn checked(db: &DbConnection, kanji: &str, kana: &str) -> String {
    unchecked(db, kanji, kana)
        .and_then(|furi| {
            let furi_parsed = from_str(&furi).map(|i| i.kana).join("");
            (furi_parsed.to_hiragana() == kana.to_hiragana()).then(|| furi)
        })
        .unwrap_or(furigana_block(kanji, kana))
}

/// Generate a furigana string. Returns None on error
pub fn unchecked(db: &DbConnection, kanji: &str, kana: &str) -> Option<String> {
    let furis = calc_kanji_readings(kanji, kana)?;
    Some(
        furigana_to_str(
            |ji, na| retrieve_readings(db, ji, na),
            kanji,
            furis.into_iter(),
        )
        .join(""),
    )
}

/// Returns the readings of a kanji by its literal
fn get_kanji(db: &DbConnection, l: &str) -> Option<(Option<Vec<String>>, Option<Vec<String>>)> {
    use crate::schema::kanji::dsl::*;

    let mut lock = KANJICACHE.lock().unwrap();
    if let Some(cache) = lock.cache_get(&l.to_owned()) {
        return Some(cache.to_owned());
    }

    let readings: (Option<Vec<String>>, Option<Vec<String>>) = kanji
        .select((kunyomi, onyomi))
        .filter(literal.eq(l))
        .get_result(db)
        .ok()?;

    lock.cache_set(l.to_owned(), readings.clone());

    Some(readings)
}

/// Takes a kanji(compound) and the assigned kana reading and returns (hopefully) a list of the
/// provided kanji with the
fn retrieve_readings(db: &DbConnection, kanji: &str, kana: &str) -> Option<Vec<(String, String)>> {
    // If both have len of 2 the readings are obv
    if real_string_len(kanji) == real_string_len(kana) && real_string_len(kanji) == 2 {
        return Some(
            kanji
                .chars()
                .zip(kana.chars())
                .map(|(kanji, kana)| (kanji.to_string(), kana.to_string()))
                .collect(),
        );
    }

    let kanji_items = kanji.chars().filter(|i| i.is_kanji()).collect_vec();
    if kanji_items.len() == 1 {
        return Some(vec![(kanji.to_owned(), kana.to_owned())]);
    }
    if kanji_items.is_empty() {
        return None;
    }

    let kanji_readings = kanji_items
        .iter()
        .map(|i| {
            let (kun, on) = get_kanji(db, &i.to_string())?;
            let kun = kun.map(|i| format_readings(i)).unwrap_or_default();
            let on = on.map(|i| format_readings(i)).unwrap_or_default();
            let mut readings = kun.into_iter().chain(on).collect_vec();
            readings.sort_unstable();
            readings.dedup();
            Some(readings)
        })
        .collect::<Option<Vec<_>>>()?;

    let readings_list: Vec<(char, Vec<String>)> = kanji_items
        .into_iter()
        .zip(kanji_readings.into_iter())
        .collect();

    find_kanji_combo(readings_list, kana)
}

/// Find the exact readings of a kanji literal within a kanji compound
fn find_kanji_combo(
    readings_map: Vec<(char, Vec<String>)>,
    kana: &str,
) -> Option<Vec<(String, String)>> {
    let mut routes: Vec<(usize, Vec<String>, &str)> = Vec::new();
    routes.push((0, vec![], &kana));

    for (pos, (_, readings)) in readings_map.iter().enumerate() {
        let route_pos = pos + 1;
        let last_routes = routes
            .clone()
            .into_iter()
            .filter(|i| i.0 == pos)
            .collect_vec();

        if last_routes.is_empty() {
            return None;
        }

        for route in last_routes.iter() {
            let pref = find_prefix(&readings, &route.2);
            for pref in pref {
                let mut curr_route_readings = route.1.clone();
                curr_route_readings.push(pref.clone());
                let new_route = (
                    route_pos,
                    curr_route_readings,
                    &route.2[pref.bytes().len()..],
                );
                routes.push(new_route);
            }
        }
    }

    let valid_routes = routes
        .iter()
        .filter(|i| i.2.is_empty())
        .cloned()
        .collect_vec();

    let valid_routes = if valid_routes.is_empty() && !routes.is_empty() {
        let lasti = routes.last().as_ref().clone().unwrap().2.to_owned();
        let mut last = routes.last().unwrap().to_owned();
        let last_count = routes
            .iter()
            .filter(|i| i.0 + 1 == readings_map.len())
            .count();
        // If only one last kanji reading is missing, just apply the kana char
        if last.1.len() + 1 == readings_map.len() && last_count == 1 {
            last.1.push(lasti);
            // Check if this is really the same as the kana reading
            if last.1.clone().join("") == kana {
                vec![last]
            } else {
                valid_routes
            }
        } else {
            valid_routes
        }
    } else {
        valid_routes
    };

    // No or multiple routes found should be treated as invalid
    if valid_routes.is_empty() || valid_routes.len() > 1 {
        return None;
    }

    let route = valid_routes[0].1.clone();

    Some(
        readings_map
            .into_iter()
            .map(|i| i.0.to_string())
            .zip(route)
            .collect_vec(),
    )
}

fn furigana_to_str<F>(
    mut kanji_lookup: F,
    kanji_text: &str,
    mut furi: impl Iterator<Item = (String, String)>,
) -> impl Iterator<Item = String>
where
    F: FnMut(&str, &str) -> Option<Vec<(String, String)>>,
{
    let mut text_parts = text_parts(kanji_text)
        .map(|i| i.to_owned())
        .collect::<Vec<_>>()
        .into_iter();

    std::iter::from_fn(move || {
        let curr_part = text_parts.next()?;

        if curr_part.is_kanji() || (curr_part.has_kanji() && curr_part.has_symbol()) {
            let (kanji, reading) = furi.next()?;

            if let Some(readings) = kanji_lookup(&kanji, &reading) {
                if readings.len() != kanji.chars().count() {
                    Some(furigana_block(kanji, reading))
                } else {
                    let reading = readings.into_iter().map(|i| i.1).join("|");
                    Some(furigana_block(kanji, reading))
                }
            } else {
                Some(furigana_block(kanji, reading))
            }
        } else {
            Some(curr_part.to_owned())
        }
    })
}

fn furigana_block<S: AsRef<str>>(kanji: S, kana: S) -> String {
    format!("[{}|{}]", kanji.as_ref(), kana.as_ref())
}

fn find_prefix(prefixe: &Vec<String>, text: &str) -> Vec<String> {
    prefixe
        .iter()
        .filter(|i| text.to_hiragana().starts_with(&i.to_hiragana()))
        .map(|i| i.to_owned().to_owned())
        .collect_vec()
}

fn format_readings(r: Vec<String>) -> Vec<String> {
    r.into_iter()
        .map(|i| i.replace("-", ""))
        .map(|i| {
            if i.contains('.') {
                // On kun readigs, replace everything after the '.'
                i.split('.').next().unwrap().to_owned().to_hiragana()
            } else {
                i.to_hiragana()
            }
        })
        .collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{db, DbPool};

    fn get_db() -> DbPool {
        db::connect()
    }

    const INPUT: &[(&'static str, &'static str)] = &[
        ("考える", "かんがえる"),
        ("涼しい", "すずしい"),
        ("気持ち", "きもち"),
        ("飛行機", "ひこうき"),
        ("足跡", "あしあと"),
        ("千里の道も一歩から", "せんりのみちもいっぽから"),
        ("自動販売機", "じどうはんばいき"),
        ("動物園", "どうぶつえん"),
        ("生新しい", "なまあたらしい"),
        ("前貼り", "まえばり"),
        ("置く", "おく"),
        ("びっくり", "びっくり"),
    ];

    const ITEM_RESULTS: &[&'static str] = &[
        "[考|かんが]える",
        "[涼|すず]しい",
        "[気持|き|も]ち",
        "[飛行機|ひ|こう|き]",
        "[足跡|あし|あと]",
        "[千里|せん|り]の[道|みち]も[一歩|いっぽ]から",
        "[自動販売機|じ|どう|はん|ばい|き]",
        "[動物園|どう|ぶつ|えん]",
        "[生新|なま|あたら]しい",
        //"[前貼|まえば]り",
        "[前貼|まえ|ば]り",
        "[置|お]く",
        "びっくり",
    ];

    #[test]
    fn gen_furigana_checked() {
        let db = get_db().get().unwrap();
        for (pos, (kanji, kana)) in INPUT.iter().enumerate() {
            let res = checked(&db, kanji, kana);
            assert_eq!(res, ITEM_RESULTS[pos]);
        }
    }
}
