use crate::queryable::{Deletable, FromRow, Insertable, Queryable, SQL};
use error::Error;

use deadpool_postgres::{tokio_postgres::Row, Pool};
use tokio_postgres::types::ToSql;

use super::KanjiResult;

impl SQL for Meaning {
    fn get_tablename() -> &'static str {
        "kanji_meaning"
    }
}

impl FromRow for Meaning {
    fn from_row(row: &Row, offset: usize) -> Self {
        Self {
            id: row.get(offset + 0),
            kanji_id: row.get(offset + 1),
            value: row.get(offset + 2),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Meaning {
    pub id: i32,
    pub kanji_id: i32,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NewMeaning {
    pub kanji_id: i32,
    pub value: String,
}

impl SQL for NewMeaning {
    fn get_tablename() -> &'static str {
        "kanji_meaning"
    }
}

impl Insertable<2> for NewMeaning {
    fn column_names() -> [&'static str; 2] {
        ["kanji_id", "value"]
    }

    fn fields(&self) -> [&(dyn ToSql + Sync); 2] {
        [&self.kanji_id, &self.value]
    }
}

pub async fn insert_meanings(db: &Pool, meanings: Vec<NewMeaning>) -> Result<(), Error> {
    NewMeaning::insert(db, &meanings).await?;
    Ok(())
}

pub async fn find(pool: &Pool, meaning: &str) -> Result<Vec<KanjiResult>, Error> {
    let sql_query = Meaning::select_where("value &@ $1");

    let kanji_ids: Vec<i32> = Meaning::query(pool, sql_query, &[&meaning], 0)
        .await?
        .into_iter()
        .map(|i| i.kanji_id)
        .collect();

    Ok(super::load_by_idsv2(pool, &kanji_ids).await?)
}

/// Clear all kanji meanings
pub async fn clear_meanings(db: &Pool) -> Result<(), Error> {
    Meaning::delete_all(db).await?;
    Ok(())
}
