use super::super::schema::radical;
use crate::{error::Error, parse::radicals, utils::to_option, DbConnection};
use diesel::prelude::*;
use itertools::Itertools;

#[derive(Queryable, QueryableByName, Clone, Debug, Default)]
#[table_name = "radical"]
pub struct Radical {
    pub id: i32,
    pub literal: String,
    pub alternative: Option<String>,
    pub stroke_count: i32,
    pub readings: Option<Vec<String>>,
    pub translations: Option<Vec<String>>,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "radical"]
pub struct NewRadical {
    pub id: i32,
    pub literal: String,
    pub alternative: Option<String>,
    pub stroke_count: i32,
    pub readings: Option<Vec<String>>,
    pub translations: Option<Vec<String>>,
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
            readings: to_option(r.readings.into_iter().map(|i| i.to_string()).collect_vec()),
            stroke_count: r.stroke_count,
        }
    }
}

/// Inserts a new Radical into the Db
pub fn insert(db: &DbConnection, radical: NewRadical) -> Result<(), Error> {
    diesel::insert_into(radical::table)
        .values(radical)
        .execute(db)?;
    Ok(())
}
