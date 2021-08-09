mod order;
pub mod result;

use deadpool_postgres::Pool;
use futures::{stream::FuturesUnordered, TryStreamExt};
use result::Item;

use error::Error;
use japanese::JapaneseExt;
use models::kanji::{self, KanjiResult};

use super::query::Query;
use itertools::Itertools;

fn format_query(query: &str) -> String {
    query.replace(" ", "").replace(".", "").trim().to_string()
}

/// The entry of a kanji search
pub async fn search(db: &Pool, query: &Query) -> Result<Vec<Item>, Error> {
    unimplemented!()
    /*
    let q = format_query(&query.query);

    let res = if q.is_japanese() {
        by_literals(db, &query.query).await
    } else {
        by_meaning(db, &query.query).await
    }?;

    let mut items = to_item(db, res, &query).await?;
    if !q.is_japanese() {
        items.sort_by(order::by_meaning);
    }

    Ok(items)
    */
}

/*
/// Find a kanji by its literal
async fn by_literals(db: &Pool, query: &str) -> Result<Vec<KanjiResult>, Error> {
    let kanji = query
        .chars()
        .into_iter()
        .filter_map(|i| i.is_kanji().then(|| i.to_string()))
        .collect_vec();

    let mut items = kanji::find_by_literals(db, &kanji).await?;

    // Order them by occurence in query
    items
        .sort_by(|a, b| utils::get_item_order(&kanji, &a.kanji.literal, &b.kanji.literal).unwrap());

    Ok(items)
}

/// Find kanji by mits meaning
async fn by_meaning(db: &Pool, meaning: &str) -> Result<Vec<KanjiResult>, Error> {
    Ok(kanji::meaning::find(db, meaning)
        .await?
        .into_iter()
        // TODO add paginator
        .take(5)
        .collect())
}

async fn to_item(db: &Pool, items: Vec<KanjiResult>, query: &Query) -> Result<Vec<Item>, Error> {
    Ok(items
        .into_iter()
        .map(|i| Item::from_db(db, i, query.settings.user_lang, query.settings.show_english))
        .collect::<FuturesUnordered<_>>()
        .try_collect::<Vec<_>>()
        .await?)
}
*/
