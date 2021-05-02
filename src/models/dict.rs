use std::cmp::Ordering;

use super::{super::schema::dict, kanji::Kanji};
use crate::{
    error::Error,
    japanese::JapaneseExt,
    parse::jmdict::Entry,
    parse::jmdict::{information::Information, priority::Priority},
    utils, DbPool,
};
use diesel::prelude::*;
use tokio_diesel::*;

#[derive(Queryable, QueryableByName, Clone, Debug, Default)]
#[table_name = "dict"]
pub struct Dict {
    pub id: i32,
    pub sequence: i32,
    pub reading: String,
    pub kanji: bool,
    pub no_kanji: bool,
    pub priorities: Option<Vec<Priority>>,
    pub information: Option<Vec<Information>>,
    pub kanji_info: Option<Vec<i32>>,
    pub jlpt_lvl: Option<i32>,
    pub is_main: bool,
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
    pub jlpt_lvl: Option<i32>,
    pub is_main: bool,
}

impl PartialEq for Dict {
    fn eq(&self, other: &Dict) -> bool {
        self.sequence == other.sequence && self.id == other.id
    }
}

impl Dict {
    pub fn len(&self) -> usize {
        utils::real_string_len(&self.reading)
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
        items.sort_by(|a, b| utils::get_item_order(ids, &a.id, &b.id).unwrap_or(Ordering::Equal));

        Ok(items)
    }
}

pub async fn update_jlpt(db: &DbPool, l: &str, level: i32) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;
    let seq_ids = dict
        .select(sequence)
        .filter(reading.eq(l))
        .get_results_async::<i32>(db)
        .await?;

    diesel::update(dict)
        .filter(sequence.eq_any(&seq_ids))
        .set(jlpt_lvl.eq(level))
        .execute_async(db)
        .await?;

    Ok(())
}

/// Get all Database-dict structures from an entry
pub fn new_dicts_from_entry(entry: &Entry) -> Vec<NewDict> {
    let mut found_main = false;
    let has_kanji = entry.elements.iter().any(|i| i.kanji);
    entry
        .elements
        .iter()
        .map(|item| {
            let is_main = !found_main && ((item.kanji && has_kanji) || (!has_kanji) && !item.kanji);
            if is_main {
                found_main = true;
            }
            NewDict {
                sequence: entry.sequence as i32,
                information: (!item.information.is_empty()).then(|| item.information.clone()),
                no_kanji: item.no_true_reading,
                kanji: item.kanji,
                reading: item.value.clone(),
                priorities: (!item.priorities.is_empty()).then(|| item.priorities.clone()),
                kanji_info: None,
                jlpt_lvl: None,
                is_main,
            }
        })
        .collect()
}

pub async fn load_by_ids(db: &DbPool, ids: &[i32]) -> Result<Vec<Dict>, Error> {
    use crate::schema::dict::dsl::*;
    if ids.is_empty() {
        return Ok(vec![]);
    }
    Ok(dict.filter(id.eq_any(ids)).get_results_async(db).await?)
}

/// Finds words by their exact readings and retuns a vec of their sequence ids
#[cfg(feature = "tokenizer")]
pub(crate) async fn find_by_reading(
    db: &DbPool,
    readings: &[(&str, i32, bool)],
) -> Result<Vec<(i32, i32)>, Error> {
    use crate::schema::dict::dsl::*;
    if readings.is_empty() {
        return Ok(vec![]);
    }

    let mut result = Vec::new();
    for (reading_str, start, only_kana) in readings {
        let dict_res: Result<Vec<Dict>, tokio_diesel::AsyncError> = if *only_kana {
            dict.filter(reading.eq(reading_str))
                .filter(is_main.eq(true))
                .get_results_async(db)
                .await
        } else {
            dict.filter(reading.eq(reading_str))
                .get_results_async(db)
                .await
        };

        // Don't break with error if its just a 'not found error'
        if let Err(err) = dict_res {
            match err {
                AsyncError::Error(ref e) => match e {
                    diesel::result::Error::NotFound => continue,
                    _ => return Err(err.into()),
                },
                _ => return Err(err.into()),
            }
        }
        let mut dict_res = dict_res.unwrap();

        // Order results by probability
        dict_res.sort_by(|a, b| {
            if a.is_main && !b.is_main {
                return Ordering::Less;
            } else if !a.is_main && b.is_main {
                return Ordering::Greater;
            }

            if reading_str.is_kana() {
                let a_is_kana = a.is_main && !a.kanji;
                let b_is_kana = b.is_main && !b.kanji;

                if a_is_kana && !b_is_kana {
                    return Ordering::Less;
                } else if !a_is_kana && b_is_kana {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        });

        result.extend(dict_res.into_iter().map(|i| (i.sequence, *start)));
    }

    Ok(result)
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

/// Get the min(sequence) of all dicts
pub async fn min_sequence(db: &DbPool) -> Result<i32, Error> {
    use crate::schema::dict::dsl::*;

    let res: Option<i32> = dict
        .select(diesel::dsl::min(sequence))
        .get_result_async(&db)
        .await?;

    Ok(res.unwrap_or(0))
}
