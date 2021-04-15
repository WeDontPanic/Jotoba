use super::super::schema::dict;
use crate::{
    error::Error,
    parse::jmdict::Entry,
    parse::jmdict::{information::Information, priority::Priority},
    DbPool,
};
use diesel::prelude::*;
use tokio_diesel::*;

#[derive(Queryable, Clone, Debug, Default)]
pub struct Dict {
    pub id: i32,
    pub sequence: i32,
    pub reading: String,
    pub kanji: bool,
    pub no_kanji: bool,
    pub priorities: Option<Vec<Priority>>,
    pub information: Option<Vec<Information>>,
}

impl Dict {
    pub fn len(&self) -> usize {
        self.reading.len()
    }
}

impl PartialEq for Dict {
    fn eq(&self, other: &Dict) -> bool {
        self.sequence == other.sequence && self.id == other.id
    }
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "dict"]
pub struct NewDict {
    pub sequence: i32,
    pub reading: String,
    pub kanji: bool,
    pub no_kanji: bool,
    pub priorities: Option<Vec<Priority>>,
    pub information: Option<Vec<Information>>,
}

/// Get all Database-dict structures from an entry
pub fn new_dicts_from_entry(entry: &Entry) -> Vec<NewDict> {
    entry
        .elements
        .iter()
        .map(|item| NewDict {
            sequence: entry.sequence as i32,
            information: (!item.information.is_empty()).then(|| item.information.clone()),
            no_kanji: item.no_true_reading,
            kanji: item.kanji,
            reading: item.value.clone(),
            priorities: (!item.priorities.is_empty()).then(|| item.priorities.clone()),
        })
        .collect()
}

/// Returns Ok(true) if at least one dict exists in the Db
pub async fn exists(db: &DbPool) -> Result<bool, Error> {
    use crate::schema::dict::dsl::*;

    Ok(dict
        .select((id, sequence))
        .limit(1)
        .execute_async(db)
        .await?
        == 1)
}

/// Insert multiple dicts into the database
pub async fn insert_dicts(db: &DbPool, dicts: Vec<NewDict>) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;

    diesel::insert_into(dict)
        .values(dicts)
        .execute_async(db)
        .await?;

    Ok(())
}

/// Clear all dict entries
pub async fn clear_dicts(db: &DbPool) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;
    diesel::delete(dict).execute_async(db).await?;
    Ok(())
}
