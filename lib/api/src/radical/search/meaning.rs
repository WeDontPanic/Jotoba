use engine::task::SearchTask;
use itertools::Itertools;
use japanese::JapaneseExt;
use search::{engine::words::foreign::Engine, word::order::foreign::ForeignOrder};
use types::jotoba::languages::Language;

pub fn search(query: &str, language: Language) -> Vec<char> {
    if query.len() < 2 {
        return vec![];
    }

    let mut res = search::radical::search(query);

    if res.len() > 4 {
        return res;
    }

    if japanese::guessing::could_be_romaji(query) {
        res.extend(super::jp_search::search(&query.to_hiragana()));
    } else {
        res.extend(word_search(query, language));
    }

    res
}

/// Does a kana word-search and returns some likely radicals for the given query
fn word_search(query: &str, language: Language) -> Vec<char> {
    let mut search_task: SearchTask<Engine> = SearchTask::with_language(&query, language)
        .with_custom_order(ForeignOrder::new())
        .with_limit(3);

    let kanji_retr = resources::get().kanji();
    search_task
        .find()
        .into_iter()
        .filter(|word| word.get_reading().reading == query)
        .map(|word| {
            word.get_reading()
                .reading
                .chars()
                .filter(|i| i.is_kanji())
                .collect::<Vec<_>>()
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
