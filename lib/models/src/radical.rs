use crate::queryable::{
    CheckAvailable, Deletable, FromRow, Insertable, OneQueryable, OptQueryable, SQL,
};
use deadpool_postgres::{tokio_postgres::Row, Pool};
use error::Error;
use itertools::Itertools;
use parse::radicals::{self, search_radicals};
use tokio_postgres::types::ToSql;
use utils::to_option;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Radical {
    pub id: i32,
    pub literal: String,
    pub alternative: Option<String>,
    pub stroke_count: i32,
    pub readings: Vec<String>,
    pub translations: Option<Vec<String>>,
}

impl SQL for Radical {
    fn get_tablename() -> &'static str {
        "radical"
    }
}

impl FromRow for Radical {
    fn from_row(row: &Row, offset: usize) -> Self
    where
        Self: Sized,
    {
        Self {
            id: row.get(offset + 0),
            literal: row.get(offset + 1),
            alternative: row.get(offset + 2),
            stroke_count: row.get(offset + 3),
            readings: row.get(offset + 4),
            translations: row.get(offset + 5),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NewRadical {
    pub id: i32,
    pub literal: String,
    pub alternative: Option<String>,
    pub stroke_count: i32,
    pub readings: Vec<String>,
    pub translations: Option<Vec<String>>,
}

impl SQL for NewRadical {
    fn get_tablename() -> &'static str {
        "radical"
    }
}

impl Insertable<6> for NewRadical {
    fn column_names() -> [&'static str; 6] {
        [
            "id",
            "literal",
            "alternative",
            "stroke_count",
            "readings",
            "translations",
        ]
    }

    fn fields(&self) -> [&(dyn ToSql + Sync); 6] {
        [
            &self.id,
            &self.literal,
            &self.alternative,
            &self.stroke_count,
            &self.readings,
            &self.translations,
        ]
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SearchRadical {
    pub id: i32,
    pub literal: String,
    pub stroke_count: i32,
}

impl SQL for SearchRadical {
    fn get_tablename() -> &'static str {
        "search_radical"
    }
}

impl FromRow for SearchRadical {
    fn from_row(row: &Row, offset: usize) -> Self
    where
        Self: Sized,
    {
        Self {
            id: row.get(offset + 0),
            literal: row.get(offset + 1),
            stroke_count: row.get(offset + 2),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NewSearchRadical {
    pub literal: String,
    pub stroke_count: i32,
}

impl SQL for NewSearchRadical {
    fn get_tablename() -> &'static str {
        "search_radical"
    }
}

impl Insertable<2> for NewSearchRadical {
    fn column_names() -> [&'static str; 2] {
        ["literal", "stroke_count"]
    }

    fn fields(&self) -> [&(dyn ToSql + Sync); 2] {
        [&self.literal, &self.stroke_count]
    }
}

impl From<search_radicals::SearchRadical> for NewSearchRadical {
    fn from(r: search_radicals::SearchRadical) -> Self {
        Self {
            stroke_count: r.stroke_count,
            literal: r.radical.to_string(),
        }
    }
}

impl From<radicals::Radical> for NewRadical {
    fn from(r: radicals::Radical) -> Self {
        Self {
            id: r.id,
            translations: to_option(
                r.translations
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect_vec(),
            ),
            alternative: r.alternative.map(|i| i.to_string()),
            literal: r.radical.to_string(),
            readings: r.readings.into_iter().map(|i| i.to_string()).collect_vec(),
            stroke_count: r.stroke_count,
        }
    }
}

/// Inserts a new Radical into the Db
pub async fn insert_search_radical(db: &Pool, radical: NewSearchRadical) -> Result<(), Error> {
    NewSearchRadical::insert(db, &[radical]).await?;
    Ok(())
}

/// Inserts a new Radical into the Db
pub async fn insert<T: Into<NewRadical>>(
    db: &Pool,
    radicals: impl Iterator<Item = T>,
) -> Result<(), Error> {
    NewRadical::insert(db, &radicals.map(|i| i.into()).collect::<Vec<NewRadical>>()).await?;
    Ok(())
}

/// Finds and Radical by its ID. Returns `None` if none found
pub async fn find_by_id(db: &Pool, i: i32) -> Result<Option<Radical>, Error> {
    Ok(Radical::query_opt(db, Radical::select_where("id = $1"), &[&i], 0).await?)
}

pub async fn search_radical_find_by_literal(db: &Pool, l: char) -> Result<SearchRadical, Error> {
    let query = SearchRadical::select_where("literal = $1");
    SearchRadical::query_one(db, query, &[&l.to_string()], 0).await
}

/// Clear all radical entries
pub async fn clear(db: &Pool) -> Result<u64, Error> {
    Radical::delete_all(db).await
}

/// Returns Ok(true) if at least one radical exists in the Db
pub async fn exists(db: &Pool) -> Result<bool, Error> {
    Radical::exists(db).await
}

/// Clear all searh_radical entries
pub async fn clear_search_radicals(db: &Pool) -> Result<(), Error> {
    SearchRadical::delete_all(db).await?;
    Ok(())
}

/// Returns Ok(true) if at least one search_radical exists in the Db
pub async fn search_radical_exists(db: &Pool) -> Result<bool, Error> {
    SearchRadical::exists(db).await
}
