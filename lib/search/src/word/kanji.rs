use crate::engine::{self, word::japanese::gen::GenDoc};

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

    let mode = if reading.reading.starts_with('-') {
        SearchMode::LeftVariable
    } else {
        SearchMode::RightVariable
    };

    let mut words = words_with_kanji_reading(kanji, reading_type, &reading.reading, mode).await?;
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
    rt: ReadingType,
    reading: &str,
    mode: SearchMode,
) -> Result<Vec<Word>, Error> {
    use engine::word::japanese::Find;

    // TODO: this doesn't work properly: '逸 そ.れる', '気 ケ'
    // maybe we need to adjust the actual index to contain kanji readings too (should'nt it
    // already?)

    let query_document = GenDoc::new(vec![kanji.literal.to_string()]);

    let index = engine::word::japanese::index::get();
    let doc = match DocumentVector::new(index.get_indexer(), query_document.clone()) {
        Some(s) => s,
        None => return Ok(vec![]),
    };

    let res = Find::new("", 10, 0).find_by_vec(doc).await?;
    let word_storage = resources::get().words();

    let seq_ids = res.sequence_ids();
    let wordresults = seq_ids
        .iter()
        .filter_map(|i| word_storage.by_sequence(*i).map(|i| i.to_owned()))
        .filter(|i| {
            let word_reading = &i.get_reading().reading;
            let kana = &i.reading.kana.reading;
            kana.to_hiragana().contains(&reading.to_hiragana())
                && word_reading.contains(&kanji.literal.to_string())
        })
        // Prevent loading too many
        .take(100);

    Ok(wordresults.collect())
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
