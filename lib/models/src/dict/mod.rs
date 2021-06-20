pub mod collocations;

use std::cmp::Ordering;

use super::{
    kanji::{KanjiResult, KANJICACHE},
    sense,
};
use crate::{
    queryable::{prepared_query, CheckAvailable, FromRow, SQL},
    schema::dict,
    DbConnection,
};
use deadpool_postgres::{tokio_postgres::Row, Pool};
use diesel::{
    prelude::*,
    sql_types::{Integer, Text},
};
use error::Error;
use futures::future::try_join_all;
use itertools::Itertools;
use japanese::{self, furigana, JapaneseExt};
use parse::{
    accents::PitchItem,
    jmdict::{information::Information, languages::Language, priority::Priority, Entry},
};

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
    pub accents: Option<Vec<i32>>,
    pub furigana: Option<String>,
    pub collocations: Option<Vec<i32>>,
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
    pub accents: Option<Vec<i32>>,
    pub furigana: Option<String>,
    pub collocations: Option<Vec<i32>>,
}

impl SQL for Dict {
    fn get_tablename() -> &'static str {
        "dict"
    }
}

impl FromRow for Dict {
    fn from_row(row: &Row, offset: usize) -> Self
    where
        Self: Sized,
    {
        Self {
            id: row.get(offset + 0),
            sequence: row.get(offset + 1),
            reading: row.get(offset + 2),
            kanji: row.get(offset + 3),
            no_kanji: row.get(offset + 4),
            priorities: row.get(offset + 5),
            information: row.get(offset + 6),
            kanji_info: row.get(offset + 7),
            jlpt_lvl: row.get(offset + 8),
            is_main: row.get(offset + 9),
            accents: row.get(offset + 10),
            furigana: row.get(offset + 11),
            collocations: row.get(offset + 12),
        }
    }
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

    pub fn is_empty(&self) -> bool {
        self.reading.is_empty()
    }

    /// Retrieve the kanji items of the dict's kanji info
    pub async fn load_kanji_info(&self, db: &Pool) -> Result<Vec<KanjiResult>, Error> {
        if self.kanji_info.is_none() || self.kanji_info.as_ref().unwrap().is_empty() {
            return Ok(vec![]);
        }
        let ids = self.kanji_info.as_ref().unwrap();

        // Load kanji from DB
        let mut items = super::kanji::load_by_idsv2(db, ids).await?;
        // Order items based on their occurence
        items.sort_by(|a, b| {
            utils::get_item_order(ids, &a.kanji.id, &b.kanji.id).unwrap_or(Ordering::Equal)
        });

        Ok(items)
    }

    pub fn get_accents(&self) -> Option<Vec<(&str, bool)>> {
        self.accents.as_ref().and_then(|accents| {
            if accents.is_empty() {
                return None;
            }
            japanese::accent::calc_pitch(&self.reading, accents[0])
        })
    }

    /// Loads all collocations of a dict entry
    pub async fn load_collocation(
        &self,
        pool: &Pool,
        language: Language,
    ) -> Result<(i32, Vec<(String, String)>), Error> {
        if self.collocations.is_none() || self.collocations.as_ref().unwrap().is_empty() {
            return Ok((self.sequence, vec![]));
        }

        let cc = self.collocations.as_ref().unwrap();

        // Load collocation readings
        let sql = "SELECT sequence, reading FROM dict WHERE kanji = true AND sequence = ANY($1) AND is_main = true";
        let db = pool.get().await?;
        let prepared = db.prepare_cached(sql).await?;
        let readings: Vec<(i32, String)> = db
            .query(&prepared, &[&cc])
            .await?
            .into_iter()
            .map(|i| (i.get(0), i.get(1)))
            .collect();

        // Load senses to [`readings`]
        let senses = try_join_all(
            readings
                .iter()
                .map(|(seq, _)| sense::short_glosses(pool, *seq, language)),
        )
        .await?;

        // Merge both
        let res = senses
            .into_iter()
            .map(|senses| {
                let (_, rd) = readings.iter().find(|i| i.0 == senses.0).unwrap();
                (rd.to_owned(), senses.1.join(", "))
            })
            .collect_vec();

        Ok((self.sequence, res))
    }
}

pub fn update_accents(db: &DbConnection, pitch: PitchItem) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;

    let seq = find_jp_word(db, &pitch.kanji, &pitch.kana)?;

    if seq.is_none() {
        return Ok(());
    }

    diesel::update(dict)
        .filter(sequence.eq(&seq.unwrap().sequence))
        .filter(reading.eq(&pitch.kana))
        .set(accents.eq(&pitch.pitch))
        .execute(db)?;

    Ok(())
}

pub fn find_jp_word(db: &DbConnection, kanji: &str, kana: &str) -> Result<Option<Sequence>, Error> {
    let query = include_str!("../../sql/find_jp_word.sql");
    let sequence = diesel::sql_query(query)
        .bind::<Text, _>(kanji)
        .bind::<Text, _>(kana)
        .get_result(db);

    if let Err(e) = sequence {
        match e {
            diesel::result::Error::NotFound => return Ok(None),
            _ => return Err(e.into()),
        }
    }

    Ok(sequence.unwrap())
}

#[derive(QueryableByName, Clone, Copy, Debug, PartialEq)]
pub struct Sequence {
    #[sql_type = "Integer"]
    sequence: i32,
}

pub async fn update_jlpt(db: &DbConnection, l: &str, level: i32) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;
    let seq_ids = dict
        .select(sequence)
        .filter(reading.eq(l))
        .get_results::<i32>(db)?;

    diesel::update(dict)
        .filter(sequence.eq_any(&seq_ids))
        .set(jlpt_lvl.eq(level))
        .execute(db)?;

    Ok(())
}

/// Get all Database-dict structures from an entry
pub fn new_dicts_from_entry(db: &DbConnection, entry: &Entry) -> Vec<NewDict> {
    let mut found_main = false;
    let has_kanji = entry.elements.iter().any(|i| i.kanji);
    let mut dicts: Vec<NewDict> = entry
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
                accents: None,
                furigana: None,
                collocations: None,
            }
        })
        .collect();

    // Generate furigana if necessary
    let kana = dicts
        .iter()
        .find(|i| i.reading.is_kana())
        .map(|i| i.reading.clone());
    if let Some(mut main) = dicts.iter_mut().find(|i| i.is_main && i.kanji) {
        if let Some(kana) = kana {
            let furigana =
                furigana::generate::checked(|l: String| get_kanji(db, &l), &main.reading, &kana);
            main.furigana = Some(furigana);
        }
    }

    dicts
}

fn get_kanji(db: &DbConnection, l: &str) -> Option<(Option<Vec<String>>, Option<Vec<String>>)> {
    use crate::schema::kanji::dsl::*;

    let mut lock = KANJICACHE.lock().unwrap();
    if let Some(cache) = lock.cache_get(&l.to_owned()) {
        return Some(cache.to_owned());
    }

    let readings: (Option<Vec<String>>, Option<Vec<String>>) = kanji
        .select((kunyomi, onyomi))
        .filter(literal.eq(l))
        .get_result(db)
        .ok()?;

    lock.cache_set(l.to_owned(), readings.clone());

    Some(readings)
}

pub async fn load_by_ids(db: &DbConnection, ids: &[i32]) -> Result<Vec<Dict>, Error> {
    use crate::schema::dict::dsl::*;
    if ids.is_empty() {
        return Ok(vec![]);
    }
    Ok(dict.filter(id.eq_any(ids)).get_results(db)?)
}

/// Finds words by their exact readings and retuns a vec of their sequence ids
#[cfg(feature = "tokenizer")]
pub(crate) async fn find_by_reading(
    db: &DbConnection,
    readings: &[(&str, i32, bool)],
) -> Result<Vec<(i32, i32)>, Error> {
    use crate::schema::dict::dsl::*;
    if readings.is_empty() {
        return Ok(vec![]);
    }

    let mut result = Vec::new();
    for (reading_str, start, only_kana) in readings {
        let dict_res: Result<Vec<Dict>, _> = if *only_kana {
            dict.filter(reading.eq(reading_str))
                .filter(is_main.eq(true))
                .get_results(db)
        } else {
            dict.filter(reading.eq(reading_str)).get_results(db)
        };

        // Don't break with error if its just a 'not found error'
        if let Err(err) = dict_res {
            match err {
                diesel::result::Error::NotFound => continue,
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

/// Returns true if the database contains at least one dict entry with the passed reading
pub async fn reading_existsv2(db: &Pool, r: &str) -> Result<bool, Error> {
    Dict::exists_where(db, "reading = $1", &[&r]).await
}

/// Returns true if the database contains at least one dict entry with the passed reading
pub async fn reading_exists(db: &DbConnection, r: &str) -> Result<bool, Error> {
    use crate::schema::dict::dsl::*;
    use diesel::dsl::exists;
    Ok(diesel::select(exists(dict.filter(reading.eq(r)))).get_result(db)?)
}

/// Returns sequence id of the passed word
pub async fn get_word_sequence(db: &DbConnection, r: &str) -> Result<i32, Error> {
    use crate::schema::dict::dsl::*;
    Ok(dict
        .select(sequence)
        .filter(reading.eq_all(r))
        .limit(1)
        .get_result(db)?)
}

/// Returns Ok(true) if at least one dict exists in the Db
pub async fn exists(db: &Pool) -> Result<bool, Error> {
    Dict::exists(db).await
}

/// Insert multiple dicts into the database
pub async fn insert_dicts(db: &DbConnection, dicts: Vec<NewDict>) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;

    diesel::insert_into(dict).values(dicts).execute(db)?;

    Ok(())
}

/// Clear all dict entries
pub async fn clear_dicts(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;
    diesel::delete(dict).execute(db)?;
    Ok(())
}

/// Get the min(sequence) of all dicts
pub async fn min_sequence(db: &DbConnection) -> Result<i32, Error> {
    use crate::schema::dict::dsl::*;

    let res: Option<i32> = dict.select(diesel::dsl::min(sequence)).get_result(db)?;

    Ok(res.unwrap_or(0))
}

/// Load Dictionaries of a single sequence id
pub async fn load_dictionary(db: &DbConnection, sequence_id: i32) -> Result<Vec<Dict>, Error> {
    use crate::schema::dict as dict_schema;

    Ok(dict_schema::table
        .filter(dict_schema::sequence.eq_all(sequence_id))
        .order_by(dict_schema::id)
        .get_results(db)?)
}

/// Returns furigana string for a word which contains at least one kanji. If [`r`] exists multiple
/// times in the database Ok(None) gets returned
pub async fn furigana_by_reading(db: &Pool, r: &str) -> Result<Option<String>, Error> {
    let query = "SELECT furigana FROM dict WHERE kanji = true AND reading = $1 AND is_main = true";
    let mut furi: Vec<Option<String>> = prepared_query(db, query, &[&r]).await?;

    // If nothing was found search for non main readings too!
    if furi.is_empty() {
        let query = "SELECT furigana FROM dict WHERE kanji = true AND reading = $1";
        furi = prepared_query(db, query, &[&r]).await?;
    }

    if furi.len() != 1 {
        return Ok(None);
    }

    Ok(furi[0].to_owned())
}
