use crate::{
    engine::{self, word::japanese::gen::GenDoc},
    search_order::SearchOrder,
    word::order,
};

use super::{
    super::{query::Query, SearchMode},
    ResultData, Search,
};
use error::Error;
use itertools::Itertools;
use japanese::{CharType, JapaneseExt};
use resources::models::{
    kanji::{self, Kanji, ReadingType},
    words::Word,
};
use vector_space_model::DocumentVector;

/// Runs a kanji reading search
pub(super) async fn by_reading(search: &Search<'_>) -> Result<ResultData, Error> {
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
        return alternative_reading_search(search).await;
    }
    let reading_type = reading_type.unwrap();

    /*
    let mode = if reading.reading.starts_with('-') {
        SearchMode::LeftVariable
    } else {
        SearchMode::RightVariable
    };
    */

    let mut words =
        words_with_kanji_reading(kanji, reading_type, &reading.reading, search.query).await?;
    let count = words.len();

    words.truncate(10);

    Ok(ResultData {
        count,
        words,
        ..Default::default()
    })
}

async fn words_with_kanji_reading(
    kanji: &Kanji,
    _rt: ReadingType,
    reading: &str,
    query: &Query,
) -> Result<Vec<Word>, Error> {
    use engine::word::japanese::Find;

    let res = Find::new(&kanji.literal.to_string(), 1000, 0)
        .find()
        .await?;
    let word_storage = resources::get().words();

    let seq_ids = res.sequence_ids();
    let mut wordresults = seq_ids
        .iter()
        .filter_map(|i| word_storage.by_sequence(*i).map(|i| i.to_owned()))
        .filter(|word| {
            //TODO: also check for alternative readings
            if word.reading.kanji.is_none() {
                return false;
            }
            let kanji_reading = word.reading.kanji.as_ref().unwrap().reading.clone();
            let kana = &word.reading.kana.reading;
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

            readings.unwrap().iter().any(|i| {
                i.0.contains(&kanji.literal.to_string())
                    && i.1
                        .to_hiragana()
                        .contains(&kanji.get_literal_reading(&reading).unwrap().to_hiragana())
                    && kana
                        .to_hiragana()
                        .contains(&kanji::format_reading(&reading.to_hiragana()))
            })
        })
        .take(1000)
        .collect::<Vec<_>>();

    // Sort the result
    order::new_kanji_reading_search_order(
        res.get_order_map(),
        &SearchOrder::new(query, &None),
        &mut wordresults,
    );

    Ok(super::filter_languages(wordresults.into_iter().take(10), query).collect())
}

/// Do a search without the kanji literal or reading
async fn alternative_reading_search(search: &Search<'_>) -> Result<ResultData, Error> {
    let reading = search.query.form.as_kanji_reading().unwrap();

    // Modify search query
    Search {
        query: &Query {
            query: kanji::literal_kun_reading(&reading.reading),
            ..search.query.to_owned()
        },
    }
    .do_word_search()
    .await
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
        .take(14)
        .collect::<Vec<_>>();

    Ok(kanji_literals)
}
