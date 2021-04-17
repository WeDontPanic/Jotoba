use std::cmp::Ordering;

use super::{super::schema::dict, kanji::Kanji};
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
    pub kanji_info: Option<Vec<i32>>,
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
    pub kanji_info: Option<Vec<i32>>,
}

impl PartialEq for Dict {
    fn eq(&self, other: &Dict) -> bool {
        self.sequence == other.sequence && self.id == other.id
    }
}

impl Dict {
    pub fn len(&self) -> usize {
        self.reading.chars().count()
    }

    /// Retrieve the kanji items of the dict's kanji info
    pub async fn load_kanji_info(&self, db: &DbPool) -> Result<Vec<Kanji>, Error> {
        if self.kanji_info.is_none() || self.kanji_info.as_ref().unwrap().len() == 0 {
            return Ok(vec![]);
        }
        let ids = self.kanji_info.as_ref().unwrap();

        // Load kanji from DB
        let mut items = super::kanji::load_by_ids(db, ids).await?;
        // Order items based on their occurence
        items.sort_by(|a, b| get_item_order(ids, a.id, b.id));

        Ok(items)
    }
}

/// Get the order of two elements in a vector
fn get_item_order<T>(vec: &Vec<T>, a: T, b: T) -> Ordering
where
    T: PartialEq,
{
    let mut a_pos = 0;
    let mut b_pos = 0;
    let mut found_a = false;
    let mut found_b = false;

    for i in vec {
        if *i == a {
            found_a = true;
        }
        if *i == b {
            found_b = true;
        }

        if found_a && found_b {
            break;
        }

        if !found_a {
            a_pos += 1;
        }
        if !found_b {
            b_pos += 1;
        }
    }

    a_pos.cmp(&b_pos)
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
            kanji_info: None,
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
