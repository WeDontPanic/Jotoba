use std::time::SystemTime;

use crate::{error::Error, parse::jmdict::languages::Language, DbPool};

use super::{result, search::SearchMode, word};

/// Search among all data based on the input query
pub async fn search(db: &DbPool, query: &str) -> Result<Vec<result::Item>, Error> {
    let mut results: Vec<result::Item> = Vec::new();

    let mut wordresults: Vec<result::word::Item> = Vec::new();

    let start = SystemTime::now();

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

    results.extend(
        wordresults
            .into_iter()
            .map(|word| result::Item::Word(word))
            .collect::<Vec<result::Item>>(),
    );
    println!("full search took {:?}", start.elapsed());

    Ok(results)
}
