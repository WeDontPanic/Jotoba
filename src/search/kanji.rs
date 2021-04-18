use crate::{
    error::Error,
    japanese::JapaneseExt,
    models::kanji::{self, Kanji},
    DbPool,
};

use futures::future::try_join_all;

/// Find a kanji by its literal
pub async fn by_literals(db: &DbPool, query: &str) -> Result<Vec<Kanji>, Error> {
    Ok(try_join_all(
        query
            .chars()
            .filter(|i| i.is_kanji())
            .map(|literal| kanji::find_by_literal(&db, literal.to_string())),
    )
    .await?)
}
