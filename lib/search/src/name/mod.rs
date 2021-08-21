mod order;
pub mod result;

use super::query::Query;
use error::Error;

use resources::models::names::Name;

/// Search for names
pub async fn search(query: &Query) -> Result<Vec<Name>, Error> {
    /*
    let mut ns_cache = NAME_SEARCH_CACHE.lock().await;

    if let Some(cached) = ns_cache.cache_get(&query.query.clone()) {
        return Ok(cached.clone());
    }

    let res = if query.form.is_kanji_reading() {
        search_kanji(db, &query).await?
    } else if query.query.is_japanese() {
        search_native(db, &query).await?
    } else {
        search_transcription(db, &query).await?
    };

    ns_cache.cache_set(query.query.clone(), res.clone());
    */

    Ok(vec![])
}

/*
/// Search by transcription
async fn search_transcription(db: &Pool, query: &Query) -> Result<Vec<Name>, Error> {
    let search = NameSearch::new(&db, &query.query);

    let mut items = search.search_transcription().await?;

    // Sort the results based
    order::ByTranscription::new(&query.query).sort(&mut items);

    // Limit search to 10 results
    items.truncate(10);

    Ok(items)
}

/// Search by japanese input
async fn search_native(db: &Pool, query: &Query) -> Result<Vec<Name>, Error> {
    let search = NameSearch::new(&db, &query.query);

    let mut items = search.search_native(&query.query).await?;

    // Sort the results based
    order::ByNative::new(&query.query).sort(&mut items);

    // Limit search to 10 results
    items.truncate(10);

    Ok(items)
}

/// Search by japanese input
async fn search_kanji(db: &Pool, query: &Query) -> Result<Vec<Name>, Error> {
    let search = NameSearch::new(&db, &query.query);

    let kanji = query.form.as_kanji_reading().unwrap();

    let mut items = search.kanji_search(kanji).await?;

    // Sort the results based
    order::ByKanji::new(&query.query, &kanji).sort(&mut items);

    // Limit search to 10 results
    items.truncate(10);

    Ok(items)
}
*/
