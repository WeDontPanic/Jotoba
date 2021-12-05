use super::{super::query::Query, ResultData, Search};
use crate::{
    engine::{words::native, SearchTask},
    word::order,
};

use error::Error;
use itertools::Itertools;
use japanese::{CharType, JapaneseExt};
use types::jotoba::{
    kanji::{self, Kanji, ReadingType},
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

    let reading_type = kanji.get_reading_type(&reading.reading);
    if !kanji.has_reading(&reading.reading) || reading_type.is_none() {
        return alternative_reading_search(search);
    }
    let reading_type = reading_type.unwrap();

    let (words, count) =
        words_with_kanji_reading(kanji, reading_type, &reading.reading, search.query)?;

    Ok(ResultData {
        count,
        words,
        ..Default::default()
    })
}

fn words_with_kanji_reading(
    kanji: &Kanji,
    _rt: ReadingType,
    reading: &str,
    query: &Query,
) -> Result<(Vec<Word>, usize), Error> {
    let query_str = kanji.literal.to_string();

    let mut search_task = SearchTask::<native::Engine>::new(&query_str)
        .threshold(0.1)
        .limit(query.settings.page_size as usize)
        .offset(query.page_offset);

    let literal = kanji.literal.to_string();
    let reading = reading.to_string();
    let literal_reading = kanji.get_literal_reading(&reading);
    search_task.set_result_filter(move |word| {
        if word.reading.kanji.is_none() {
            return false;
        }

        let kana = &word.reading.kana.reading;

        for kanji_reading in word.reading_iter(false) {
            let kanji_reading = kanji_reading.reading.clone();

            let readings = japanese::furigana::generate::retrieve_readings(
                &mut |i: String| {
                    let retrieve = resources::get().kanji();
                    let kanji = retrieve.by_literal(i.chars().next()?)?;
                    if kanji.onyomi.is_none() && kanji.kunyomi.is_none() {
                        return None;
                    }

                    Some((kanji.kunyomi.clone(), kanji.onyomi.clone()))
                },
                &kanji_reading,
                kana,
            );

            if readings.is_none() {
                return false;
            }

            let e = readings.unwrap().iter().any(|i| {
                i.0.contains(&literal)
                    && i.1
                        .to_hiragana()
                        .contains(&literal_reading.as_ref().unwrap().to_hiragana())
                    && kana
                        .to_hiragana()
                        .contains(&kanji::format_reading(&reading.to_hiragana()))
            });

            if e {
                return true;
            }
        }

        false
    });

    let kanji_reading = query.form.as_kanji_reading().unwrap().clone();
    search_task.set_order_fn(move |word, rel, _, _| {
        order::kanji_reading_search(word, &kanji_reading, rel)
    });

    let res = search_task.find()?;
    let len = res.len();
    let mut words = res.item_iter().cloned().collect::<Vec<_>>();

    super::filter_languages(
        words.iter_mut(),
        query.settings.user_lang,
        query.settings.show_english,
    );

    Ok((words, len))
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
pub(super) fn load_word_kanji_info(words: &[Word]) -> Result<Vec<Kanji>, Error> {
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

    Ok(kanji_literals)
}
