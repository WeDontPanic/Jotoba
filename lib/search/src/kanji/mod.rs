mod order;
pub mod result;
mod tag_only;

use self::result::KanjiResult;
use super::query::Query;
use crate::{engine::words::native::Engine, query::QueryLang, word::order::native::NativeOrder};
use engine::task::SearchTask;
use error::Error;
use jp_utils::JapaneseExt;
use result::Item;
use types::jotoba::{
    kanji::Kanji,
    search::guess::{Guess, GuessType},
};

/// The entry of a kanji search
pub fn search(query: &Query) -> Result<KanjiResult, Error> {
    if query.form.is_tag_only() {
        return tag_only::search(query);
    }

    let query_str = format_query(&query.query_str);

    let res = match query.q_lang {
        QueryLang::Japanese => by_japanese_query(&query.query_str),
        QueryLang::Korean => by_korean_reading(&query.query_str),
        QueryLang::Foreign | QueryLang::Undetected => by_meaning(&query.query_str),
    };

    // TODO: don't use this items in v2 since compound words don't need to be loaded
    // here anymore
    let mut items = to_item(res, &query);

    if !query_str.is_japanese() {
        items.sort_by(order::default);
    }

    let total_len = items.len();

    let page_size = query.settings.page_size as usize;

    let items = items
        .into_iter()
        .skip(query.page_offset(page_size))
        .take(page_size)
        .collect::<Vec<_>>();

    Ok(KanjiResult { items, total_len })
}

/// Find a kanji by its literal
fn by_japanese_query(query: &str) -> Vec<Kanji> {
    // Use kanji from query
    let kanji = kanji_from_str(query);
    if !kanji.is_empty() || query.is_kanji() {
        return kanji;
    }

    // Do word searc with kana instead
    kana_search(query)
}

/// Search for kanji using kana query
fn kana_search(query: &str) -> Vec<Kanji> {
    let q = query.to_string();

    let mut search_task = SearchTask::<Engine>::new(query)
        .with_limit(10)
        .with_threshold(0.8)
        .with_result_filter(move |i| i.has_reading(&q))
        .with_custom_order(NativeOrder::new(query.to_string()));

    search_task
        .find()
        .into_iter()
        .map(|i| kanji_from_str(&i.get_reading().reading))
        .flatten()
        .take(100)
        .collect()
}

fn by_korean_reading(query: &str) -> Vec<Kanji> {
    resources::get()
        .kanji()
        .iter()
        .filter(|k| k.korean_h.iter().any(|kw| kw == query))
        .cloned()
        .collect()
}

#[inline]
fn from_char(c: char) -> Option<Kanji> {
    resources::get().kanji().by_literal(c).cloned()
}

fn kanji_from_str(text: &str) -> Vec<Kanji> {
    text.chars()
        .into_iter()
        .filter_map(|i| i.is_kanji().then(|| from_char(i)).flatten())
        .take(100)
        .collect()
}

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    let query_str = &query.query_str;

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
    // TODO: implement proper algo kek
    let meaning = meaning.to_lowercase();
    resources::get()
        .kanji()
        .iter()
        .filter(|i| i.meanings.contains(&meaning))
        .cloned()
        .collect::<Vec<_>>()
}

#[inline]
fn to_item(items: Vec<Kanji>, query: &Query) -> Vec<Item> {
    items
        .into_iter()
        .map(|i| Item::load_words(i, query.lang_param()))
        .collect()
}

#[inline]
fn format_query(query: &str) -> String {
    query.replace(" ", "").replace(".", "").trim().to_string()
}
