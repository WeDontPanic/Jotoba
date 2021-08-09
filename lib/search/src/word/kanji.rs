use super::{
    super::{query::Query, search_order::SearchOrder, SearchMode},
    order, ResultData, Search,
};
use error::Error;
use futures::{stream::FuturesUnordered, TryStreamExt};
use itertools::Itertools;
use japanese::{CharType, JapaneseExt};
use models::{
    dict::Dict,
    kanji::{self, KanjiResult},
};
use resources::models::{kanji::Kanji, words::Word};
use utils::{self, to_option};

const MAX_KANJI_INFO_ITEMS: usize = 5;

/// Runs a kanji reading search
pub(super) async fn by_reading(search: &Search<'_>) -> Result<ResultData, Error> {
    /*
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

    SearchOrder::new(search.query, &None).sort(&mut w, order::kanji_reading_search);

    let count = w.len();
    w.truncate(10);

    Ok(ResultData {
        words: w,
        count,
        ..Default::default()
    })
    */
    unimplemented!()
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

    return Ok(kanji_literals);
}
