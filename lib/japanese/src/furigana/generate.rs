use super::{
    super::{text_parts, JapaneseExt},
    calc_kanji_readings, from_str,
};
use crate::utils::real_string_len;
use itertools::Itertools;

pub trait RetrieveKanji:
    FnMut(String) -> Option<(Option<Vec<String>>, Option<Vec<String>>)>
{
}
impl<T: FnMut(String) -> Option<(Option<Vec<String>>, Option<Vec<String>>)> + ?Sized> RetrieveKanji
    for T
{
}

/// Encodes furigana readings of a japanese word/sentence and returns it in form of a string which
/// can be parsed later on
pub fn checked<R: RetrieveKanji>(retrieve: R, kanji: &str, kana: &str) -> String {
    unchecked(retrieve, kanji, kana)
        .and_then(|furi| {
            let furi_parsed = from_str(&furi).map(|i| i.kana).join("");
            (furi_parsed.to_hiragana() == kana.to_hiragana()).then(|| furi)
        })
        .unwrap_or_else(|| furigana_block(kanji, kana))
}

/// Generate a furigana string. Returns None on error
pub fn unchecked<R: RetrieveKanji>(mut retrieve: R, kanji: &str, kana: &str) -> Option<String> {
    let furis = calc_kanji_readings(kanji, kana)?;
    Some(
        furigana_to_str(
            |ji, na| retrieve_readings(&mut retrieve, ji, na),
            kanji,
            furis.into_iter(),
        )
        .join(""),
    )
}

/// Takes a kanji(compound) and the assigned kana reading and returns (hopefully) a list of the
/// provided kanji with the
pub fn retrieve_readings<R: RetrieveKanji>(
    retrieve: &mut R,
    kanji: &str,
    kana: &str,
) -> Option<Vec<(String, String)>> {
    // If both have len of 2 the readings are obv
    if real_string_len(kanji) == real_string_len(kana) {
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
            let (kun, on) = retrieve(i.to_string())?;
            let kun = kun.map(format_readings).unwrap_or_default();
            let on = on.map(format_readings).unwrap_or_default();
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
    let mut routes: Vec<(usize, Vec<String>, &str)> = vec![(0, vec![], kana)];

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
            let pref = find_prefix(&readings, route.2);
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
        let lasti = routes.last().as_ref().unwrap().2.to_owned();
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
            Some(curr_part)
        }
    })
}

fn furigana_block<S: AsRef<str>>(kanji: S, kana: S) -> String {
    format!("[{}|{}]", kanji.as_ref(), kana.as_ref())
}

fn find_prefix(prefixe: &[String], text: &str) -> Vec<String> {
    prefixe
        .iter()
        .filter(|i| text.to_hiragana().starts_with(&i.to_hiragana()))
        .cloned()
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
