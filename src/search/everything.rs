use itertools::Itertools;
use std::time::SystemTime;

use crate::{error::Error, japanese, parse::jmdict::languages::Language, DbPool};

use super::{result, search::SearchMode, word};

/// Search among all data based on the input query
pub async fn search(db: &DbPool, query: &str) -> Result<Vec<result::Item>, Error> {
    let mut results: Vec<result::Item> = Vec::new();
    let start = SystemTime::now();

    let (native_word_res, gloss_word_res): (Vec<result::word::Item>, Vec<result::word::Item>) = futures::try_join!(
        search_word_by_native(db, query),
        search_word_by_glosses(db, query)
    )?;

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
    if !japanese::is_japanese(query) {
        return Ok(vec![]);
    }

    let mut wordresults: Vec<result::word::Item> = Vec::new();

    let mut exact_words = word::WordSearch::new(db, query)
        .with_language(Language::German)
        .with_mode(SearchMode::Exact)
        .search_native()
        .await?;
    exact_words.sort();
    // Search for exact matches

    let mut right_variable = word::WordSearch::new(db, query)
        .with_language(Language::German)
        .with_mode(SearchMode::RightVariable)
        .search_native()
        .await?;

    right_variable.retain(|i| !exact_words.contains(&i));
    right_variable.sort();
    right_variable.truncate(10);

    // Search for right variable
    wordresults.extend(exact_words);
    wordresults.extend(right_variable);

    Ok(wordresults)
}

/// Search gloss readings
pub async fn search_word_by_glosses(
    db: &DbPool,
    query: &str,
) -> Result<Vec<result::word::Item>, Error> {
    if japanese::is_japanese(query) {
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

    let (mut exact_words, mut case_ignoring) =
        futures::try_join!(ws1.search_by_glosses(), ws2.search_by_glosses())?;

    exact_words.sort();
    case_ignoring.sort();
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
