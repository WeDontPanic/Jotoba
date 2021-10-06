mod order;
pub mod result;

use itertools::Itertools;
use resources::models::kanji::Kanji;
use result::Item;

use error::Error;
use japanese::JapaneseExt;

use crate::{
    engine::{
        guess::{Guess, GuessType},
        words::native,
        SearchTask,
    },
    query::QueryLang,
};

use super::query::Query;

/// The entry of a kanji search
pub fn search(query: &Query) -> Result<Vec<Item>, Error> {
    let query_str = format_query(&query.query);

    let res;

    if query.language == QueryLang::Japanese {
        res = by_literals(&query.query)
    } else {
        res = by_meaning(&query.query)
    };

    let mut items = to_item(res, &query);
    if !query_str.is_japanese() {
        items.sort_by(order::by_meaning);
    }

    Ok(items)
}

/// Find a kanji by its literal
fn by_literals(query: &str) -> Vec<Kanji> {
    let kanji = all_kanji_from_text(query);
    if !kanji.is_empty() || query.is_kanji() {
        return kanji;
    }

    // kana search

    let search = SearchTask::<native::Engine>::new(query).threshold(0.89);
    let res = search.find_exact().unwrap_or_default();
    if res.is_empty() {
        return vec![];
    }

    let text = res
        .into_iter()
        .filter(|i| i.item.reading.kana.reading == query)
        .map(|i| i.item.get_reading().reading.chars().collect::<Vec<_>>())
        .flatten()
        .take(8)
        .unique()
        .join("");

    all_kanji_from_text(&text)
}

fn all_kanji_from_text(text: &str) -> Vec<Kanji> {
    let kanji_storage = resources::get().kanji();

    text.chars()
        .into_iter()
        .filter(|i| i.is_kanji())
        .filter_map(|literal| kanji_storage.by_literal(literal))
        .cloned()
        .take(15)
        .collect()
}

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    let query_str = &query.query;

    let kanji_storage = resources::get().kanji();
    let guess = query_str
        .chars()
        .into_iter()
        .filter(|i| i.is_kanji())
        .filter_map(|literal| kanji_storage.by_literal(literal))
        .take(15)
        .count();

    Some(Guess::new(guess as u32, GuessType::Accurate))
}

/// Find kanji by mits meaning
fn by_meaning(meaning: &str) -> Vec<Kanji> {
    let mut out = Vec::new();

    let kanji_storage = resources::get().kanji();

    // TODO: implement a proper meaning search algorithm
    for kanji in kanji_storage.iter() {
        if kanji.meanings.contains(&meaning.to_string()) {
            out.push(kanji.clone());
        }
    }

    out
}

#[inline]
fn to_item(items: Vec<Kanji>, query: &Query) -> Vec<Item> {
    items
        .into_iter()
        .map(|i| Item::load_words(i, query.settings.user_lang, query.settings.show_english))
        .collect()
}

#[inline]
fn format_query(query: &str) -> String {
    query.replace(" ", "").replace(".", "").trim().to_string()
}
