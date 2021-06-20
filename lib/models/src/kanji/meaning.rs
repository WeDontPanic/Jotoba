use crate::{
    queryable::{Deletable, FromRow, Queryable, SQL},
    schema::kanji_meaning,
    DbConnection,
};
use diesel::RunQueryDsl;
use error::Error;

use deadpool_postgres::{tokio_postgres::Row, Pool};

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

#[derive(Queryable, QueryableByName, Clone, Debug, Default, PartialEq)]
#[table_name = "kanji_meaning"]
pub struct Meaning {
    pub id: i32,
    pub kanji_id: i32,
    pub value: String,
}

#[derive(Insertable, Clone, Debug, PartialEq, Default)]
#[table_name = "kanji_meaning"]
pub struct NewMeaning {
    pub kanji_id: i32,
    pub value: String,
}

pub async fn insert_meanings(db: &DbConnection, meanings: Vec<NewMeaning>) -> Result<(), Error> {
    use crate::schema::kanji_meaning::dsl::*;
    diesel::insert_into(kanji_meaning)
        .values(meanings)
        .execute(db)?;
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
