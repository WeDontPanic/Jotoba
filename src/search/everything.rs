use std::time::SystemTime;

use crate::{error::Error, DbPool};

use super::{result, word};

/// Search among all data based on the input query
pub async fn search(db: &DbPool, query: &str) -> Result<Vec<result::Item>, Error> {
    let mut results: Vec<result::Item> = Vec::new();

    // Search for words and add to result
    /*
    results.extend(
        search_word(db, query)
            .await?
            .into_iter()
            .map(|i| result::Item::Word(i))
            .collect::<Vec<result::Item>>(),
    );*/

    results.extend(
        word::WordSearch::new(db, query, super::search::SearchMode::Exact)
            .with_language(crate::parse::jmdict::languages::Language::German)
            .search_native()
            .await?
            .into_iter()
            .map(|word| result::Item::Word(word))
            .collect::<Vec<result::Item>>(),
    );

    let word_find = word::WordSearch::new(db, query, super::search::SearchMode::RightVariable)
        .with_language(crate::parse::jmdict::languages::Language::German)
        .search_native()
        .await?;

    let start = SystemTime::now();
    results.extend(
        word_find
            .into_iter()
            .map(|word| result::Item::Word(word))
            .collect::<Vec<result::Item>>(),
    );
    println!("searh took {:?}", start.elapsed());

    Ok(results)
}
