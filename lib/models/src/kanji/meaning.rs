use crate::{schema::kanji_meaning, sql::ExpressionMethods, DbConnection};
use error::Error;

use super::KanjiResult;

use diesel::prelude::*;

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

pub async fn find(db: &DbConnection, meaning: &str) -> Result<Vec<KanjiResult>, Error> {
    use crate::schema::kanji_meaning::dsl::*;

    let kanji_ids = kanji_meaning
        .select(kanji_id)
        .filter(value.text_search(meaning))
        .get_results(db)?;

    Ok(super::load_by_ids(db, &kanji_ids).await?)
}

/// Clear all kanji meanings
pub async fn clear_meanings(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::kanji_meaning::dsl::*;
    diesel::delete(kanji_meaning).execute(db)?;
    Ok(())
}
