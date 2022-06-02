use error::Error;
use itertools::Itertools;
use japanese::JapaneseExt;
use search::engine::{self, SearchTask};
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
        res.extend(word_search(query, language).unwrap_or_default());
    }

    res
}

/// Does a kana word-search and returns some likely radicals for the given query
fn word_search(query: &str, language: Language) -> Result<Vec<char>, Error> {
    let mut search_task: SearchTask<engine::words::foreign::Engine> =
        SearchTask::with_language(&query, language)
            .limit(3)
            .threshold(0.8f32);

    let foreign_order = search::word::order::foreign::ForeignOrder::new();
    search_task.set_order_fn(move |word, rel, q_str, lang| {
        //search::word::order::foreign::(word, rel, q_str, lang.unwrap(), language)
        foreign_order.score(word, rel, q_str, lang.unwrap(), language)
    });

    let kanji_retr = resources::get().kanji();
    let res = search_task
        .find()?
        .into_iter()
        .filter(|word| word.word.get_reading().reading == query)
        .map(|i| {
            println!("{}", i.word.get_reading().reading);
            i.word
                .get_reading()
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
        .collect::<Vec<_>>();

    Ok(res)
}
