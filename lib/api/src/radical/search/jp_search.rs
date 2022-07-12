use std::collections::HashSet;

use itertools::Itertools;
use japanese::JapaneseExt;
use search::engine::{words::native, SearchTask};
use types::{api::radical::search::KanjiRads, jotoba::kanji::Kanji};

/// Returns a list of radicals based on the radical-search `query`
pub fn search(query: &str) -> Vec<char> {
    if japanese::JapaneseExt::has_kanji(query) {
        return kanji_search(query);
    }

    kana_search(query)
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
        out.push(kanji.into());
        dups.insert(kanji.literal);

        for part in kanji.parts.iter() {
            let mut kanji_w_r = resources::get().kanji().by_radicals(&[*part]);
            kanji_w_r.sort_by(|a, b| a.stroke_count.cmp(&b.stroke_count));
            for k in kanji_w_r.into_iter().take(10) {
                if k.stroke_count < kanji.stroke_count || dups.contains(&k.literal) {
                    continue;
                }
                dups.insert(k.literal);
                out.push(k.into());
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

/// Takes all kanji from `query` and returns a list of all unique radicals to build all kanji
/// picked from `query`
fn kanji_search(query: &str) -> Vec<char> {
    query
        .chars()
        .map(|k| kanji_radicals(k))
        .flatten()
        .unique()
        .collect()
}

#[inline]
fn kanji_radicals(kanji: char) -> Vec<char> {
    get_kanji(kanji)
        .map(|i| i.parts.clone())
        .unwrap_or_default()
}

/// Does a kana word-search and returns some likely radicals for the given query
fn kana_search(query: &str) -> Vec<char> {
    let mut search_task: SearchTask<native::Engine> = SearchTask::new(&query).limit(3);

    let original_query = query.to_string();
    search_task.with_custom_order(move |item| {
        search::word::order::japanese_search_order(item, Some(&original_query))
    });

    search_task
        .find()
        .into_iter()
        .map(|i| i.get_reading().reading.chars().filter(|i| i.is_kanji()))
        .flatten()
        .unique()
        .map(|kanji| kanji_radicals(kanji))
        .flatten()
        .unique()
        .take(10)
        .collect()
}
