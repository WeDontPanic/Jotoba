use itertools::Itertools;
use japanese::JapaneseExt;
use search::engine::{self, SearchTask};

/// Returns a list of radicals based on the radical-search `query`
pub fn search(query: &str) -> Vec<char> {
    if japanese::JapaneseExt::has_kanji(query) {
        return kanji_search(query);
    }

    kana_search(query)
}

/// Takes all kanji from `query` and returns a list of all unique radicals to build all kanji
/// picked from `query`
fn kanji_search(query: &str) -> Vec<char> {
    let kanji_retr = resources::get().kanji();

    query
        .chars()
        .filter_map(|kanji| kanji_retr.by_literal(kanji).map(|i| &i.parts))
        .flatten()
        .copied()
        .unique()
        .collect()
}

/// Does a kana word-search and returns some likely radicals for the given query
fn kana_search(query: &str) -> Vec<char> {
    let mut search_task: SearchTask<engine::words::native::Engine> =
        SearchTask::new(&query).limit(3).threshold(0.8f32);

    let original_query = query.to_string();
    search_task.with_custom_order(move |item| {
        search::word::order::japanese_search_order(item, Some(&original_query))
    });

    let kanji_retr = resources::get().kanji();
    search_task
        .find()
        .into_iter()
        .map(|i| {
            i.get_reading()
                .reading
                .chars()
                .filter(|i| i.is_kanji())
                .collect::<Vec<char>>()
        })
        .flatten()
        .unique()
        .filter_map(|kanji| kanji_retr.by_literal(kanji).map(|i| &i.parts))
        .flatten()
        .unique()
        .copied()
        .take(10)
        .collect()
}
