mod order;
pub mod result;

use result::Item;

use crate::{
    error::Error,
    japanese::JapaneseExt,
    models::kanji::{self, Kanji},
    utils, DbPool,
};

use super::query::Query;
use diesel::sql_types::Text;
use futures::future::join_all;
use itertools::Itertools;
use tokio_diesel::AsyncRunQueryDsl;

/// The entry of a kanji search
pub async fn search(db: &DbPool, query: &Query) -> Result<Vec<Item>, Error> {
    if query.query.is_japanese() {
        by_literals(db, &query).await
    } else {
        by_meaning(db, &query).await
    }
}

/// Find a kanji by its literal
async fn by_literals(db: &DbPool, query: &Query) -> Result<Vec<Item>, Error> {
    let kanji = query
        .query
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
async fn by_meaning(db: &DbPool, query: &Query) -> Result<Vec<Item>, Error> {
    let items: Vec<Kanji> = diesel::sql_query("select * from find_kanji_by_meaning($1)")
        .bind::<Text, _>(&query.query)
        .get_results_async(db)
        .await
        .unwrap();

    let mut res = to_item(db, items).await?;
    res.sort_by(order::by_meaning);
    Ok(res)
}

async fn to_item(db: &DbPool, items: Vec<Kanji>) -> Result<Vec<Item>, Error> {
    Ok(join_all(
        items
            .into_iter()
            .map(|i| Item::from_db(db, i))
            .collect::<Vec<_>>(),
    )
    .await)
}
