use std::time::SystemTime;

use super::word::lower;
use super::{result_order, Search, SearchMode};
use crate::{cache::SharedCache, error::Error, japanese::JapaneseExt, models::name::Name, DbPool};
use async_std::sync::Mutex;
use once_cell::sync::Lazy;
use tokio_diesel::*;

/// An in memory Cache for search results
static NAME_SEARCH_CACHE: Lazy<Mutex<SharedCache<String, Vec<Name>>>> =
    Lazy::new(|| Mutex::new(SharedCache::with_capacity(1000)));

/// Defines the structure of a
/// name based search
#[derive(Clone)]
pub struct NameSearch<'a> {
    search: Search<'a>,
    db: &'a DbPool,
    limit: i64,
}

/// Search for names
pub async fn search(db: &DbPool, query: &str) -> Result<Vec<Name>, Error> {
    let mut ns_cache = NAME_SEARCH_CACHE.lock().await;

    if let Some(cached) = ns_cache.cache_get(&query.to_owned()) {
        return Ok(cached.clone());
    }

    let res = if query.is_japanese() {
        search_native(db, query).await?
    } else {
        search_transcription(db, query).await?
    };

    ns_cache.cache_set(query.to_owned(), res.clone());

    Ok(res)
}

async fn search_transcription(db: &DbPool, query: &str) -> Result<Vec<Name>, Error> {
    let mut search = NameSearch::new(&db, Search::new(&query, SearchMode::Variable));

    if query.len() < 4 {
        search.with_limit(100);
        search.search.mode = SearchMode::Exact;
    }

    let mut items = search.search_transcription().await?;

    let start = SystemTime::now();
    // Sort the results based
    result_order::NameSearchTranscription::new(query).sort(&mut items);
    println!("order took: {:?}", start.elapsed());

    // Limit search to 10 results
    items.truncate(10);

    Ok(items)
}

async fn search_native(db: &DbPool, query: &str) -> Result<Vec<Name>, Error> {
    let mut search = NameSearch::new(&db, Search::new(&query, SearchMode::Variable));

    if query.len() < 4 {
        search.with_limit(100);
        search.search.mode = SearchMode::Exact;
    }

    let mut items = search.search_native().await?;

    let start = SystemTime::now();
    // Sort the results based
    result_order::NameSearchNative::new(query).sort(&mut items);
    println!("order took: {:?}", start.elapsed());

    // Limit search to 10 results
    items.truncate(10);

    Ok(items)
}

impl<'a> NameSearch<'a> {
    pub fn new(db: &'a DbPool, search: Search<'a>) -> Self {
        Self {
            search,
            db,
            limit: 0,
        }
    }

    pub fn with_limit(&mut self, limit: i64) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Search name by transcription
    pub async fn search_transcription(&self) -> Result<Vec<Name>, Error> {
        use crate::schema::name::dsl::*;
        use diesel::prelude::*;

        let query = self.search.query;
        let like_pred = self.search.mode.to_like(query);

        Ok(name
            .filter(transcription.like(&like_pred))
            .get_results_async(&self.db)
            .await?)
    }

    /// Search name by japanese
    pub async fn search_native(&self) -> Result<Vec<Name>, Error> {
        use crate::schema::name::dsl::*;
        use diesel::prelude::*;

        let query = self.search.query.clone().to_lowercase();
        let like_pred = self.search.mode.to_like(&query);

        if self.limit == 0 {
            Ok(if query.is_kanji() {
                // Only need to search in kana
                name.filter(kanji.like(&like_pred))
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_kana() {
                // Only need to search in kanji
                name.filter(kana.like(&like_pred))
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_japanese() {
                // Search in both, kana & kanji
                name.filter(kanji.like(&like_pred).or(kana.like(&like_pred)))
                    .get_results_async(&self.db)
                    .await?
            } else {
                // Search in transcriptions
                name.filter(lower(transcription).like(&like_pred))
                    .get_results_async(&self.db)
                    .await?
            })
        } else {
            Ok(if query.is_kanji() {
                // Only need to search in kana
                name.filter(kanji.like(&like_pred))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_kana() {
                // Only need to search in kanji
                name.filter(kana.like(&like_pred))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_japanese() {
                // Search in both, kana & kanji
                name.filter(kanji.like(&like_pred).or(kana.like(&like_pred)))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else {
                // Search in transcriptions
                name.filter(lower(transcription).like(&like_pred))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            })
        }
    }
}
