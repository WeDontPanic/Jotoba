use crate::{
    schema::{radical, search_radical},
    DbConnection,
};
use diesel::prelude::*;
use error::Error;
use itertools::Itertools;
use parse::radicals::{self, search_radicals};
use utils::to_option;

#[derive(Queryable, QueryableByName, Clone, Debug, Default, PartialEq)]
#[table_name = "radical"]
pub struct Radical {
    pub id: i32,
    pub literal: String,
    pub alternative: Option<String>,
    pub stroke_count: i32,
    pub readings: Vec<String>,
    pub translations: Option<Vec<String>>,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "radical"]
pub struct NewRadical {
    pub id: i32,
    pub literal: String,
    pub alternative: Option<String>,
    pub stroke_count: i32,
    pub readings: Vec<String>,
    pub translations: Option<Vec<String>>,
}

#[derive(Queryable, QueryableByName, Clone, Debug, Default, PartialEq)]
#[table_name = "search_radical"]
pub struct SearchRadical {
    pub id: i32,
    pub literal: String,
    pub stroke_count: i32,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "search_radical"]
pub struct NewSearchRadical {
    pub literal: String,
    pub stroke_count: i32,
}

impl From<search_radicals::SearchRadical> for NewSearchRadical {
    fn from(r: search_radicals::SearchRadical) -> Self {
        Self {
            stroke_count: r.stroke_count,
            literal: r.radical.to_string(),
        }
    }
}

impl<'a> From<radicals::Radical<'a>> for NewRadical {
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
pub fn insert_search_radical(db: &DbConnection, radical: NewSearchRadical) -> Result<(), Error> {
    diesel::insert_into(search_radical::table)
        .values(radical)
        .execute(db)?;
    Ok(())
}

/// Inserts a new Radical into the Db
pub fn insert(db: &DbConnection, radical: NewRadical) -> Result<(), Error> {
    diesel::insert_into(radical::table)
        .values(radical)
        .execute(db)?;
    Ok(())
}

pub async fn find_by_id(db: &DbConnection, i: i32) -> Result<Radical, Error> {
    use crate::schema::radical::dsl::*;
    Ok(radical.filter(id.eq(i)).get_result(db)?)
}

pub async fn search_radical_find_by_literal(
    db: &DbConnection,
    l: char,
) -> Result<SearchRadical, Error> {
    use crate::schema::search_radical::dsl::*;
    Ok(search_radical
        .filter(literal.eq(l.to_string()))
        .get_result(db)?)
}

/// Clear all radical entries
pub async fn clear(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::radical::dsl::*;
    diesel::delete(radical).execute(db)?;
    Ok(())
}

/// Returns Ok(true) if at least one radical exists in the Db
pub async fn exists(db: &DbConnection) -> Result<bool, Error> {
    use crate::schema::radical::dsl::*;
    Ok(radical.select(id).limit(1).execute(db)? == 1)
}

/// Clear all searh_radical entries
pub async fn clear_search_radicals(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::search_radical::dsl::*;
    diesel::delete(search_radical).execute(db)?;
    Ok(())
}

/// Returns Ok(true) if at least one search_radical exists in the Db
pub async fn search_radical_exists(db: &DbConnection) -> Result<bool, Error> {
    use crate::schema::search_radical::dsl::*;
    Ok(search_radical.select(id).limit(1).execute(db)? == 1)
}
