use jp_utils::JapaneseExt;
use search::radical::word::RomajiSearch;
use std::collections::{HashMap, HashSet};
use types::{api::app::radical::search::KanjiRads, jotoba::kanji::Kanji};

/// Returns a list of radicals based on the radical-search `query`
pub fn search(query: &str) -> HashSet<char> {
    if query.has_kanji() {
        return kanji_search(query);
    }

    RomajiSearch::new(query).run()
}

/// Returns a List of kanji that use similar radicals as the query.
pub fn similar_kanji_search(query: &str) -> Vec<KanjiRads> {
    let kanji = query
        .chars()
        .filter(|i| i.is_kanji())
        .filter_map(|lit| get_kanji(lit));

    let mut dups: HashSet<char> = HashSet::new();
    let mut out: Vec<KanjiRads> = Vec::new();

    for kanji in kanji {
        // Add written kanji to the result too
        out.push(into_kanji_rads(kanji));
        dups.insert(kanji.literal);

        for part in kanji.parts.iter() {
            let mut kanji_w_r = resources::get().kanji().by_radicals(&[*part]);
            kanji_w_r.sort_by(|a, b| a.stroke_count.cmp(&b.stroke_count));
            for k in kanji_w_r.into_iter().take(10) {
                if k.stroke_count < kanji.stroke_count || dups.contains(&k.literal) {
                    continue;
                }
                dups.insert(k.literal);
                out.push(into_kanji_rads(k));
            }
        }
    }

    out.truncate(50);
    out
}

#[inline]
fn get_kanji(lit: char) -> Option<&'static Kanji> {
    resources::get().kanji().by_literal(lit)
}

/// Convert a kanji to a `KanjiRads`
fn into_kanji_rads(kanji: &Kanji) -> KanjiRads {
    let mut rads: HashMap<u32, Vec<char>> = HashMap::with_capacity(kanji.parts.len());
    for part in &kanji.parts {
        let stroke_count = japanese::radicals::get_stroke_count(*part);
        if let Some(stroke_count) = stroke_count {
            rads.entry(stroke_count).or_default().push(*part);
        }
    }
    KanjiRads::new(kanji.literal, rads)
}

/// Takes all kanji from `query` and returns a list of all unique radicals to build all kanji
/// picked from `query`
#[inline]
fn kanji_search(query: &str) -> HashSet<char> {
    query.chars().map(|k| kanji_radicals(k)).flatten().collect()
}

#[inline]
fn kanji_radicals(kanji: char) -> Vec<char> {
    get_kanji(kanji)
        .map(|i| i.parts.clone())
        .unwrap_or_default()
}
/*
/// Does a kana word-search and returns some likely radicals for the given query
fn kana_search(query: &str) -> HashSet<char> {
    let mut search_task: SearchTask<Engine> = SearchTask::new(&query)
        .with_limit(3)
        .with_threshold(0.8)
        .with_custom_order(NativeOrder::new(query.to_string()));

    search_task
        .find()
        .into_iter()
        .map(|i| i.get_reading().reading.chars().filter(|i| i.is_kanji()))
        .flatten()
        .unique()
        .map(|kanji| kanji_radicals(kanji))
        .flatten()
        .take(10)
        .collect()
} */
