use std::time::SystemTime;

use crate::{
    error::Error, japanese::JapaneseExt, search::sentence::sentencesearch::SentenceSearch, DbPool,
};

use super::query::Query;
use futures::future::try_join_all;

mod order;
pub mod result;
mod sentencesearch;

/// Searches for sentences
pub async fn search(db: &DbPool, query: &Query) -> Result<Vec<result::Item>, Error> {
    if query.query.is_japanese() {
        search_jp(db, query).await
    } else {
        search_foreign(db, query).await
    }
}

/// Searches for sentences (jp input)
pub async fn search_jp(db: &DbPool, query: &Query) -> Result<Vec<result::Item>, Error> {
    let search = SentenceSearch::new(db, &query.query, query.settings.user_lang);
    let sentences = search.by_jp().await?;

    let items = try_join_all(sentences.clone().into_iter().map(|i| i.into_item(&db))).await?;

    Ok(items)
}

/// Searches for sentences (other input)
pub async fn search_foreign(db: &DbPool, query: &Query) -> Result<Vec<result::Item>, Error> {
    let search = SentenceSearch::new(db, &query.query, query.settings.user_lang);
    let sentences = search.by_foreign().await?;
    let items = try_join_all(sentences.clone().into_iter().map(|i| i.into_item(&db))).await?;
    Ok(items)
}
