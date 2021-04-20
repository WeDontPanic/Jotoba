use async_std::sync::{Mutex, MutexGuard};
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::time::SystemTime;

use crate::{
    cache::SharedCache, error::Error, japanese::JapaneseExt, models::kanji,
    parse::jmdict::languages::Language, DbPool,
};

use super::{
    result,
    result_order::{GlossWordOrder, NativeWordOrder},
    word, SearchMode,
};

/// An in memory Cache for search results
static SEARCH_CACHE: Lazy<Mutex<SharedCache<String, Vec<result::Item>>>> =
    Lazy::new(|| Mutex::new(SharedCache::with_capacity(1000)));

const MAX_KANJI_INFO_ITEMS: usize = 5;

/// Search among all data based on the input query
pub async fn search(db: &DbPool, query: &str) -> Result<Vec<result::Item>, Error> {
    let start = SystemTime::now();

    // Lock cache
    let mut search_cache: MutexGuard<SharedCache<String, Vec<result::Item>>> =
        SEARCH_CACHE.lock().await;

    // Try to use cached value
    if let Some(c_res) = search_cache.cache_get(&query.to_owned()) {
        println!("cached search took {:?}", start.elapsed());
        return Ok(c_res.clone());
    }

    // Perform (word) searches asynchronously
    let (native_word_res, gloss_word_res): (Vec<result::word::Item>, Vec<result::word::Item>) = futures::try_join!(
        search_word_by_native(db, query),
        search_word_by_glosses(db, query)
    )?;

    // Chain native and word results into one vector
    let word_results = native_word_res
        .into_iter()
        .chain(gloss_word_res)
        .collect_vec();

    // Chain and map results into one result vector
    let results = load_word_kanji_info(db, &word_results)
        .await?
        .into_iter()
        .map(|i| i.into())
        .collect::<Vec<result::Item>>()
        .into_iter()
        .chain(word_results.into_iter().map(|i| i.into()).collect_vec())
        .collect_vec();

    println!("full search took {:?}", start.elapsed());

    // Set cache for future usage
    search_cache.cache_set(query.to_owned(), results.clone());

    Ok(results)
}

/// Perform a native word search
pub async fn search_word_by_native(
    db: &DbPool,
    query: &str,
) -> Result<Vec<result::word::Item>, Error> {
    if !query.is_japanese() || query.is_empty() {
        return Ok(vec![]);
    }

    // Define basic search structure
    let mut word_search = word::WordSearch::new(db, query);
    word_search.with_language(Language::German);

    // Perform the word search
    let mut wordresults = if query.chars().count() <= 2 && query.is_kana() {
        // Search for exact matches only if query.len() <= 2
        let res = word_search
            .with_mode(SearchMode::Exact)
            .search_native()
            .await?;

        if res.is_empty() {
            // Do another search if no exact result was found
            word_search
                .with_mode(SearchMode::RightVariable)
                .search_native()
                .await?
        } else {
            res
        }
    } else {
        word_search
            .with_mode(SearchMode::RightVariable)
            .search_native()
            .await?
    };

    // Sort the results based
    NativeWordOrder::new(query).sort(&mut wordresults);

    // Limit search to 10 results
    wordresults.truncate(10);

    Ok(wordresults)
}

/// load word assigned kanji
pub async fn load_word_kanji_info(
    db: &DbPool,
    words: &Vec<result::word::Item>,
) -> Result<Vec<kanji::Kanji>, Error> {
    use futures::future::join_all;

    let res: Vec<Vec<kanji::Kanji>> = join_all(
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
            // Load kanji from DB
            .map(|word| word.load_kanji_info(db)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<Vec<kanji::Kanji>>, Error>>()?;

    // if first word with kanji reading has more
    // than MAX_KANJI_INFO_ITEMS kanji, display all of them only
    let limit = {
        if !res.is_empty() && res[0].len() > MAX_KANJI_INFO_ITEMS {
            res[0].len()
        } else {
            MAX_KANJI_INFO_ITEMS
        }
    };

    // Remove duplicates
    let mut items_new = Vec::new();
    res.into_iter()
        .flatten()
        .collect_vec()
        .into_iter()
        .for_each(|i| {
            if !items_new.contains(&i) {
                items_new.push(i);
            }
        });

    // Limit result and map to result::Item
    Ok(items_new.into_iter().take(limit).collect_vec())
}

/// Search gloss readings
pub async fn search_word_by_glosses(
    db: &DbPool,
    query: &str,
) -> Result<Vec<result::word::Item>, Error> {
    if query.is_japanese() {
        return Ok(vec![]);
    }

    // TODO don't make exact search
    let mut wordresults = word::WordSearch::new(db, query)
        .with_language(Language::German)
        .with_case_insensitivity(true)
        .with_mode(SearchMode::Exact)
        .search_by_glosses()
        .await?;

    // Sort the results based
    GlossWordOrder::new(query).sort(&mut wordresults);

    // Limit search to 10 results
    wordresults.truncate(10);

    Ok(wordresults)
}
