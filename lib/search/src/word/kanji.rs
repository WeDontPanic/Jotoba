use super::{super::query::Query, ResultData, Search};
use crate::{
    engine::{words::native::k_reading, SearchTask},
    word::order,
};
use error::Error;
use itertools::Itertools;
use japanese::CharType;
use types::jotoba::{
    kanji::{self, Kanji},
    words::Word,
};

/// Runs a kanji reading search
pub(super) fn by_reading(search: &Search<'_>) -> Result<ResultData, Error> {
    let reading = search
        .query
        .form
        .as_kanji_reading()
        .ok_or(Error::Undefined)?;

    let kanji_storage = resources::get().kanji();

    let kanji = kanji_storage
        .by_literal(reading.literal)
        .ok_or(Error::Undefined)?;

    if !kanji.has_reading(&reading.reading) {
        return alternative_reading_search(search);
    }

    let engine_query = format!("{}{}", kanji.literal, reading.reading);
    Ok(reading_search(search.query, &engine_query))
}

fn reading_search(query: &Query, engine_query: &str) -> ResultData {
    let limit = query.settings.page_size as usize;
    let offset = query.page_offset;

    let mut search_task = SearchTask::<k_reading::Engine>::new(&engine_query)
        .limit(limit)
        .offset(offset);

    search_task.with_custom_order(move |item| order::kanji_reading_search(item));

    let res = search_task.find();
    let len = res.len();
    let mut words = res.into_iter().cloned().collect::<Vec<_>>();

    let lang = query.settings.user_lang;
    let show_english = query.settings.show_english;
    super::filter_languages(words.iter_mut(), lang, show_english);

    ResultData::new(words, len)
}

/// Do a search without the kanji literal or reading
fn alternative_reading_search(search: &Search<'_>) -> Result<ResultData, Error> {
    let reading = search.query.form.as_kanji_reading().unwrap();

    // Modify search query
    Search {
        query: &Query {
            query: kanji::literal_kun_reading(&reading.reading),
            ..search.query.to_owned()
        },
    }
    .do_word_search()
}

/// Load word assigned kanji
pub fn load_word_kanji_info(words: &[Word]) -> Vec<Kanji> {
    let kanji_resources = resources::get().kanji();

    let kanji_literals = words
        .iter()
        .filter_map(|i| {
            let kanji = &i.reading.kanji.as_ref()?.reading;
            Some(japanese::all_words_with_ct(kanji, CharType::Kanji))
        })
        .flatten()
        .map(|i| i.chars().collect::<Vec<_>>())
        .flatten()
        .filter_map(|i| kanji_resources.by_literal(i).cloned())
        .unique_by(|i| i.literal)
        .take(10)
        .collect::<Vec<_>>();

    kanji_literals
}
