use itertools::Itertools;

/// Returns a list of radicals based on the radical-search `query`
pub fn search(query: &str) -> Vec<char> {
    if japanese::JapaneseExt::has_kanji(query) {
        return kanji_search(query);
    }

    // TODO:
    vec![]
}

/// Takes all kanji from `query` and returns a list of all unique radicals to build all kanji
/// picked from `query`
fn kanji_search(query: &str) -> Vec<char> {
    let kanji_retr = resources::get().kanji();

    query
        .chars()
        .filter_map(|kanji| kanji_retr.by_literal(kanji).and_then(|i| i.parts.as_ref()))
        .flatten()
        .copied()
        .unique()
        .collect::<Vec<char>>()
}
