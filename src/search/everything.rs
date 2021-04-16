use itertools::Itertools;
use std::time::SystemTime;

use crate::{error::Error, japanese::JapaneseExt, parse::jmdict::languages::Language, DbPool};

use super::{result, result_order::NativeWordOrder, word, SearchMode};

/// Search among all data based on the input query
pub async fn search(db: &DbPool, query: &str) -> Result<Vec<result::Item>, Error> {
    let mut results: Vec<result::Item> = Vec::new();

    let start = SystemTime::now();

    // Perform searches asynchronously
    let (native_word_res, gloss_word_res): (Vec<result::word::Item>, Vec<result::word::Item>) = futures::try_join!(
        search_word_by_native(db, query),
        search_word_by_glosses(db, query)
    )?;

    if !native_word_res.is_empty() {
        if let Some(fw) = native_word_res.iter().find(|i| i.reading.kanji.is_some()) {
            let kanji = fw
                .reading
                .kanji
                .as_ref()
                .unwrap()
                .load_kanji_info(db)
                .await?;

            results.extend(
                kanji
                    .into_iter()
                    .map(|i| result::Item::Kanji(i))
                    .rev()
                    .collect_vec(),
            );
        }
    }

    results.extend(
        native_word_res
            .into_iter()
            .map(|i| result::Item::Word(i))
            .collect_vec(),
    );

    results.extend(
        gloss_word_res
            .into_iter()
            .map(|i| result::Item::Word(i))
            .collect_vec(),
    );

    println!("full search took {:?}", start.elapsed());

    Ok(results)
}

/// Perform a native word search
pub async fn search_word_by_native(
    db: &DbPool,
    query: &str,
) -> Result<Vec<result::word::Item>, Error> {
    if !query.is_japanese() {
        return Ok(vec![]);
    }

    let mut wordresults: Vec<result::word::Item> = Vec::new();

    let mut word_search = word::WordSearch::new(db, query);
    word_search.with_language(Language::German);

    // Search for exact matches
    let exact_words = word_search
        .with_mode(SearchMode::Exact)
        .search_native()
        .await?;

    let right_variable = word_search
        .with_mode(SearchMode::RightVariable)
        .search_native()
        .await?
        .into_iter()
        // remove already existing elements
        .filter(|i| !exact_words.contains(&i))
        .collect_vec();

    // Search for right variable
    wordresults.extend(exact_words);
    wordresults.extend(right_variable);

    NativeWordOrder::new(query).sort(&mut wordresults);

    // Limit search to 10 results
    wordresults.truncate(10);

    Ok(wordresults)
}

/// Search gloss readings
pub async fn search_word_by_glosses(
    db: &DbPool,
    query: &str,
) -> Result<Vec<result::word::Item>, Error> {
    if query.is_japanese() {
        return Ok(vec![]);
    }

    let mut wordresults: Vec<result::word::Item> = Vec::new();

    let mut ws1 = word::WordSearch::new(db, query);
    ws1.with_language(Language::German)
        .with_mode(SearchMode::Exact);

    let mut ws2 = word::WordSearch::new(db, query);
    ws2.with_language(Language::German)
        .with_case_insensitivity(true)
        .with_mode(SearchMode::Exact);

    let (exact_words, mut case_ignoring) =
        futures::try_join!(ws1.search_by_glosses(), ws2.search_by_glosses())?;

    // TODO sort results

    /*
    exact_words.sort();
    case_ignoring.sort();
    */
    case_ignoring.retain(|i| !exact_words.contains(&i));

    // Search for exact matches

    /*
    let mut right_variable = word::WordSearch::new(db, query)
        .with_language(Language::German)
        .with_mode(SearchMode::RightVariable)
        .search_by_glosses()
        .await?;

    right_variable.retain(|i| !exact_words.contains(&i));
    right_variable.sort();
    right_variable.truncate(10);
    wordresults.extend(right_variable);
    */

    // Search for right variable
    wordresults.extend(exact_words);
    wordresults.extend(case_ignoring);

    Ok(wordresults)
}
