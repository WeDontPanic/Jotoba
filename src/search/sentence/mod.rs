use crate::{
    error::Error, japanese::JapaneseExt, search::sentence::sentencesearch::SentenceSearch, DbPool,
};

use super::query::Query;

mod order;
pub mod result;
mod sentencesearch;

/// Searches for sentences
pub async fn search(db: &DbPool, query: &Query) -> Result<Vec<result::Sentence>, Error> {
    if query.query.is_japanese() {
        search_jp(db, query).await
    } else {
        search_foreign(db, query).await
    }
}

/// Searches for sentences (jp input)
pub async fn search_jp(db: &DbPool, query: &Query) -> Result<Vec<result::Sentence>, Error> {
    let search = SentenceSearch::new(db, &query.query, query.settings.user_lang);
    Ok(search.by_jp().await?)
}

/// Searches for sentences (other input)
pub async fn search_foreign(db: &DbPool, query: &Query) -> Result<Vec<result::Sentence>, Error> {
    let search = SentenceSearch::new(db, &query.query, query.settings.user_lang);
    Ok(search.by_foreign().await?)
}
