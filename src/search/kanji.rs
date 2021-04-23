use crate::{
    error::Error,
    japanese::JapaneseExt,
    models::kanji::{self, Kanji},
    search::result::kanji::Item,
    utils, DbPool,
};

use diesel::sql_types::Text;
use futures::future::join_all;
use itertools::Itertools;
use tokio_diesel::AsyncRunQueryDsl;

use super::result_order::order_kanji_by_meaning;

/// A kanji search. Automatically decides whether to search for meaning or literal
pub async fn search(db: &DbPool, query: &str) -> Result<Vec<Item>, Error> {
    if query.is_japanese() {
        by_literals(db, query).await
    } else {
        by_meaning(db, query).await
    }
}

/// Find a kanji by its literal
pub async fn by_literals(db: &DbPool, query: &str) -> Result<Vec<Item>, Error> {
    let kanji = query
        .chars()
        .into_iter()
        .filter_map(|i| i.is_kanji().then(|| i.to_string()))
        .collect_vec();

    let mut items = kanji::find_by_literals(db, &kanji).await?;

    // Order them by occurence in query
    items.sort_by(|a, b| utils::get_item_order(&kanji, &a.literal, &b.literal).unwrap());

    to_item(db, items).await
}

/// Find kanji by mits meaning
pub async fn by_meaning(db: &DbPool, query: &str) -> Result<Vec<Item>, Error> {
    let items: Vec<Kanji> = diesel::sql_query("select * from find_kanji_by_meaning($1)")
        .bind::<Text, _>(query)
        .get_results_async(db)
        .await
        .unwrap();

    let mut res = to_item(db, items).await?;
    res.sort_by(order_kanji_by_meaning);
    Ok(res)
}

pub async fn to_item(db: &DbPool, items: Vec<Kanji>) -> Result<Vec<Item>, Error> {
    Ok(join_all(
        items
            .into_iter()
            .map(|i| Item::from_db(db, i))
            .collect::<Vec<_>>(),
    )
    .await)
}
