mod order;
pub mod result;

use resources::models::kanji::Kanji;
use result::Item;

use error::Error;
use japanese::JapaneseExt;

use super::query::Query;

/// The entry of a kanji search
pub async fn search(query: &Query) -> Result<Vec<Item>, Error> {
    let query_str = format_query(&query.query);

    let res;

    if query_str.is_japanese() {
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
    let kanji_storage = resources::get().kanji();

    query
        .chars()
        .into_iter()
        .filter(|i| i.is_kanji())
        .filter_map(|literal| kanji_storage.by_literal(literal))
        .cloned()
        .collect()
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
