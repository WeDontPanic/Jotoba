use crate::{
    error::Error, japanese::JapaneseExt, models::kanji, search::result::kanji::Item, DbPool,
};

use futures::future::{join_all, try_join_all};

/// Find a kanji by its literal
pub async fn by_literals(db: &DbPool, query: &str) -> Result<Vec<Item>, Error> {
    let items = try_join_all(
        query
            .chars()
            .filter(|i| i.is_kanji())
            .map(|literal| kanji::find_by_literal(&db, literal.to_string())),
    )
    .await?;

    Ok(join_all(
        items
            .into_iter()
            .map(|i| Item::from_db(db, i))
            .collect::<Vec<_>>(),
    )
    .await)
}
