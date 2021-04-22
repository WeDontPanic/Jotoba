use crate::{
    error::Error,
    japanese::JapaneseExt,
    models::kanji::{self, Kanji},
    search::result::kanji::Item,
    DbPool,
};

use futures::future::{join_all, try_join_all};
use itertools::Itertools;

/// Find a kanji by its literal
pub async fn by_literals(db: &DbPool, query: &str) -> Result<Vec<Item>, Error> {
    let kanji = query
        .chars()
        .into_iter()
        .filter_map(|i| i.is_kanji().then(|| i.to_string()))
        .collect_vec();

    let items = kanji::find_by_literals(db, &kanji).await?;

    Ok(join_all(
        items
            .into_iter()
            .map(|i| Item::from_db(db, i))
            .collect::<Vec<_>>(),
    )
    .await)
}
