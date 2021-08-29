mod order;
pub mod result;

use super::query::Query;
use error::Error;

use japanese::JapaneseExt;
use resources::models::names::Name;

/// Search for names
pub async fn search(query: &Query) -> Result<Vec<Name>, Error> {
    let res = if query.form.is_kanji_reading() {
        search_kanji(&query).await?
    } else if query.query.is_japanese() {
        search_native(&query).await?
    } else {
        search_transcription(&query).await?
    };

    Ok(res)
}

/// Search by transcription
async fn search_transcription(query: &Query) -> Result<Vec<Name>, Error> {
    unimplemented!()
}

/// Search by japanese input
async fn search_native(query: &Query) -> Result<Vec<Name>, Error> {
    let resources = resources::get().names();

    use crate::engine::name::japanese::Find;

    let names = Find::new(&query.query, 10, 0)
        .find()
        .await?
        .retrieve_ordered(|seq_id| resources.by_sequence(seq_id as u32).cloned())
        .collect();

    Ok(names)
}

/// Search by japanese input
async fn search_kanji(query: &Query) -> Result<Vec<Name>, Error> {
    unimplemented!()
}
