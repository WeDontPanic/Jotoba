pub mod traits;

use std::collections::HashSet;

pub use traits::ReadingRetrieve;

use super::super::{text_parts, JapaneseExt};
use super::map_readings;
use super::parse;
use crate::utils::real_string_len;
use itertools::Itertools;

/// Generates furigana readings for the given `kanji` input based on the provided `kana` reading and
/// kanji readings which are being passed using `retrieve`. In case a reading can't be correctly
/// identified, the full kanji<->kana furigana block is getting returned instead of an error.
pub fn checked<R: ReadingRetrieve>(retrieve: R, kanji: &str, kana: &str) -> String {
    let unchecked_furi = match unchecked(retrieve, kanji, kana) {
        Some(u) => u,
        None => return furigana_block(kanji, kana),
    };

    let furi_parsed = parse::from_str(&unchecked_furi).map(|i| i.kana).join("");

    // check if built correctly
    check(&furi_parsed, kana)
        .then(|| unchecked_furi)
        // if not correct use one block for all
        .unwrap_or_else(|| furigana_block(kanji, kana))
}

fn check(gen: &str, kana: &str) -> bool {
    let gen = gen
        .chars()
        .filter(|c| !c.is_symbol())
        .collect::<String>()
        .to_hiragana();
    let kana = kana
        .chars()
        .filter(|c| !c.is_symbol())
        .collect::<String>()
        .to_hiragana();
    gen == kana
}

/// Generates furigana readings for the given `kanji` input based on the provided `kana` reading and
/// kanji readings which are being passed using `retrieve`
pub fn unchecked<R: ReadingRetrieve>(retrieve: R, kanji: &str, kana: &str) -> Option<String> {
    let kanji_mappings = map_readings(kanji, kana)?;
    Some(gen_iter(retrieve, kanji, kanji_mappings).join(""))
}

/// Returns an iterator over all encoded furigana parts
pub fn gen_iter<'a, R>(
    retrieve: R,
    kanji_text: &'a str,
    readings: Vec<(String, String)>,
) -> impl Iterator<Item = String> + 'a
where
    R: ReadingRetrieve + 'a,
{
    let mut text_parts = text_parts(kanji_text);
    let mut furi = readings.into_iter();
    std::iter::from_fn(move || {
        let curr_part = text_parts.next()?;

        // No need to encode kana parts
        if !curr_part.is_kanji() {
            return Some(curr_part.to_string());
        }

        let (kanji, reading) = furi.next()?;
        if let Some(readings) = assign_readings(&retrieve, &kanji, &reading) {
            if readings.len() != kanji.chars().count() {
                return Some(furigana_block(kanji, reading));
            }

            let reading = readings.into_iter().map(|i| i.1).join("|");
            return Some(furigana_block(kanji, reading));
        }

        Some(furigana_block(kanji, reading))
    })
}

/// Takes a kanji(compound) and the assigned kana reading and returns (hopefully) a list of the
/// provided kanji with the
pub fn assign_readings<R: ReadingRetrieve>(
    retrieve: R,
    kanji: &str,
    kana: &str,
) -> Option<Vec<(String, String)>> {
    let kanji_len = real_string_len(kanji);
    let kana_len = real_string_len(kana);

    // If both have len of 2 the readings are obv
    if kanji_len == kana_len {
        return Some(
            kanji
                .chars()
                .zip(kana.chars())
                .map(|(kanji, kana)| (kanji.to_string(), kana.to_string()))
                .collect(),
        );
    }

    let kanji_lits = get_kanji_literals(kanji);
    if kanji_lits.len() == 1 {
        return Some(vec![(kanji.to_owned(), kana.to_owned())]);
    }

    let kanji_readings = kanji_lits
        .iter()
        .map(|i| (*i, format_readings(retrieve.all(*i))))
        .collect::<Vec<_>>();

    if kanji_readings.is_empty() {
        return None;
    }

    find_kanji_combo(kanji_readings, kana)
}

/// Find the exact readings of a kanji literal within a kanji compound
fn find_kanji_combo(
    readings_map: Vec<(char, HashSet<String>)>,
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

pub fn furigana_block<S: AsRef<str>, T: AsRef<str>>(kanji: S, kana: T) -> String {
    format!("[{}|{}]", kanji.as_ref(), kana.as_ref())
}

fn get_kanji_literals(inp: &str) -> Vec<char> {
    inp.chars().filter(|i| i.is_kanji()).collect()
}

fn find_prefix(prefixe: &HashSet<String>, text: &str) -> Vec<String> {
    prefixe
        .iter()
        .filter(|i| text.to_hiragana().starts_with(&i.to_hiragana()))
        .cloned()
        .collect_vec()
}

fn format_readings(r: Vec<String>) -> HashSet<String> {
    r.into_iter()
        .map(|i| i.replace("-", ""))
        .map(|i| {
            if i.contains('.') {
                // On kun readigs, replace everything after the '.'
                let fmt1 = i.split('.').next().unwrap().to_owned().to_hiragana();
                let fmt2 = i.replace('.', "").to_hiragana();
                vec![fmt1, fmt2]
            } else {
                vec![i.to_hiragana()]
            }
        })
        .flatten()
        .collect()
}
