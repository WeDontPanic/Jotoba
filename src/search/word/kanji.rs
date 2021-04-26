use super::{result::Word, Search};
use crate::{
    error::Error,
    japanese::JapaneseExt,
    models::{
        dict::Dict,
        kanji::{self, Kanji as DbKanji},
    },
    search::{query::KanjiReading, query::Query, utils},
};
use futures::future::try_join_all;
use itertools::Itertools;

const MAX_KANJI_INFO_ITEMS: usize = 5;

pub(super) async fn reading<'a>(
    search: &Search<'a>,
    reading: &KanjiReading,
) -> Result<Vec<Word>, Error> {
    let kanji = kanji::find_by_literal(&search.db, reading.literal.to_string()).await?;

    let reading_type = kanji.get_reading_type(&reading.reading);
    if !kanji.has_reading(&reading.reading) || reading_type.is_none() {
        // only search for reading
        return Search {
            db: search.db,
            query: &Query {
                query: kanji::kun_literal_reading(&reading.reading),
                ..search.query.to_owned()
            },
        }
        .do_word_search()
        .await;
    }
    let reading_type = reading_type.unwrap();

    Ok(vec![])
}

/// load word assigned kanji
pub(super) async fn load_word_kanji_info<'a>(
    search: &Search<'a>,
    words: &Vec<Word>,
) -> Result<Vec<DbKanji>, Error> {
    let kanji_words = get_kanji_words(words);

    let retrieved_kanji = {
        // Also show kanji even if no word was found
        if !kanji_words.is_empty() {
            try_join_all(
                kanji_words
                    .iter()
                    .map(|word| word.load_kanji_info(&search.db)),
            )
            .await?
            .into_iter()
            .flatten()
            .collect_vec()
        } else {
            // No words found, search only for kanji from query
            try_join_all(search.query.query.chars().into_iter().filter_map(|i| {
                i.is_kanji()
                    .then(|| crate::models::kanji::find_by_literal(&search.db, i.to_string()))
            }))
            .await?
        }
    };

    // If first word with kanji reading has more
    // than MAX_KANJI_INFO_ITEMS kanji, display all of them only
    let limit = {
        if !kanji_words.is_empty() && kanji_words[0].reading.kanji_count() > MAX_KANJI_INFO_ITEMS {
            kanji_words[0].reading.kanji_count()
        } else {
            MAX_KANJI_INFO_ITEMS
        }
    };

    // Limit result and map to result::Item
    Ok(utils::remove_dups(retrieved_kanji)
        .into_iter()
        .take(limit)
        .collect_vec())
}

/// Returns first 10 dicts of words which have a kanji
fn get_kanji_words(words: &Vec<Word>) -> Vec<&Dict> {
    words
        .iter()
        // Filter only words with kanji readings
        .filter_map(|i| {
            i.reading
                .kanji
                .is_some()
                .then(|| i.reading.kanji.as_ref().unwrap())
        })
        // Don't load too much
        .take(10)
        .collect_vec()
}
