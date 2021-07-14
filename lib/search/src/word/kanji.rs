use super::{
    super::{query::Query, search_order::SearchOrder, SearchMode},
    order,
    result::Word,
    ResultData, Search, WordSearch,
};
use error::Error;
use futures::{stream::FuturesUnordered, TryStreamExt};
use itertools::Itertools;
use japanese::JapaneseExt;
use models::{
    dict::Dict,
    kanji::{self, KanjiResult},
};
use utils::{self, to_option};

const MAX_KANJI_INFO_ITEMS: usize = 5;

/// Runs a kanji reading search
pub(super) async fn by_reading(search: &Search<'_>) -> Result<ResultData, Error> {
    let reading = search
        .query
        .form
        .as_kanji_reading()
        .ok_or(Error::Undefined)?;

    let kanji = kanji::find_by_literalv2(&search.pool, reading.literal.to_string()).await?;
    if kanji.is_none() {
        return alternative_reading_search(search).await;
    }
    let kanji = kanji.unwrap();

    let reading_type = kanji.kanji.get_reading_type(&reading.reading);
    if !kanji.kanji.has_reading(&reading.reading) || reading_type.is_none() {
        return alternative_reading_search(search).await;
    }

    let mode = if reading.reading.starts_with('-') {
        SearchMode::LeftVariable
    } else {
        SearchMode::RightVariable
    };

    let mut seq_ids = kanji
        .kanji
        .find_readings(search.pool, reading, reading_type.unwrap(), mode, true)
        .await?;

    // Do 2nd search if 1st didn't return enough
    if seq_ids.len() <= 2 {
        seq_ids = kanji
            .kanji
            .find_readings(
                search.pool,
                reading,
                reading_type.unwrap(),
                SearchMode::Variable,
                false,
            )
            .await?;
    }

    // If still nothing was found return
    if seq_ids.is_empty() {
        return alternative_reading_search(search).await;
    }

    let (mut w, _) = WordSearch::load_words_by_seq(
        search.pool,
        &seq_ids,
        search.query.settings.user_lang,
        search.query.settings.show_english,
        &to_option(search.query.get_part_of_speech_tags()),
        |_| (),
    )
    .await?;

    #[cfg(feature = "tokenizer")]
    let search_order = SearchOrder::new(search.query, &None);

    #[cfg(not(feature = "tokenizer"))]
    let search_order = SearchOrder::new(search.query);

    search_order.sort(&mut w, order::kanji_reading_search);

    let count = w.len();
    w.truncate(10);

    Ok(ResultData {
        words: w,
        count,
        ..Default::default()
    })
}

/// Do a search without the kanji literal or reading
pub(super) async fn alternative_reading_search(search: &Search<'_>) -> Result<ResultData, Error> {
    let reading = search.query.form.as_kanji_reading().unwrap();

    // Modify search query
    Search {
        pool: search.pool,
        query: &Query {
            query: kanji::gen_readings::literal_reading(&reading.reading),
            ..search.query.to_owned()
        },
    }
    .do_word_search()
    .await
}

/// load word assigned kanji
pub(super) async fn load_word_kanji_info(
    search: &Search<'_>,
    words: &[Word],
) -> Result<Vec<KanjiResult>, Error> {
    let kanji_words = get_kanji_words(words);
    let retrieved_kanji = {
        // Also show kanji even if no word was found
        // TODO make only one DB query for this
        if !kanji_words.is_empty() {
            kanji_words
                .iter()
                .map(|word| word.load_kanji_info(&search.pool))
                .collect::<FuturesUnordered<_>>()
                .try_collect::<Vec<_>>()
                .await?
                .into_iter()
                .flatten()
                .collect_vec()
        } else {
            search
                .query
                .query
                .chars()
                .into_iter()
                .filter_map(|i| {
                    i.is_kanji()
                        .then(|| models::kanji::find_by_literalv2(&search.pool, i.to_string()))
                })
                .collect::<FuturesUnordered<_>>()
                .try_collect::<Vec<_>>()
                .await?
                .into_iter()
                .filter_map(|i| i)
                .collect_vec()
        }
    };

    // If first word with kanji reading has more
    // than MAX_KANJI_INFO_ITEMS kanji, display all of them only
    let limit = {
        if !kanji_words.is_empty() && kanji_words[0].reading.kanji_count() > words.len() {
            kanji_words[0].reading.kanji_count()
        } else {
            words.len()
        }
    };

    // Limit result and map to result::Item
    Ok(utils::remove_dups(retrieved_kanji)
        .into_iter()
        .take(limit)
        .collect())
}

/// Returns first 10 dicts of words which have a kanji
fn get_kanji_words(words: &[Word]) -> Vec<&Dict> {
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
        .collect()
}
