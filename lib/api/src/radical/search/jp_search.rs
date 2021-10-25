use itertools::Itertools;

pub fn search(query: &str) -> Vec<char> {
    if japanese::JapaneseExt::has_kanji(query) {
        return kanji_search(query);
    }

    // TODO:
    vec![]
}

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
