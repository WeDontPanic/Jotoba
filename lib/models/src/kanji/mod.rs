pub mod gen_readings;
pub mod meaning;
pub mod reading;

use std::mem::take;

use async_trait::async_trait;
use deadpool_postgres::{tokio_postgres::Row, Pool};
use reading::KanjiReading;

use crate::queryable::{prepared_query, FromRow, FromRows, Queryable, SQL};

use super::{
    dict::{self, Dict},
    kanji::meaning::{Meaning, NewMeaning},
    radical::{self, Radical, SearchRadical},
};
use crate::{
    schema::{kanji, kanji_element},
    search_mode::SearchMode,
    DbConnection,
};
use cache::SharedCache;
use error::Error;
use parse::{
    kanji_ele::KanjiPart,
    kanjidict::{self, Character},
};
use utils::to_option;

use async_std::sync::{Mutex, MutexGuard};
use diesel::{
    prelude::*,
    sql_types::{Bool, Text},
};
use futures::future::try_join_all;
use itertools::Itertools;
use once_cell::sync::Lazy;
use romaji::RomajiExt;

/// An in memory Cache for kanji items
static KANJI_RESULT_CACHE: Lazy<Mutex<SharedCache<i32, KanjiResult>>> =
    Lazy::new(|| Mutex::new(SharedCache::with_capacity(10000)));

#[derive(diesel::Queryable, QueryableByName, Clone, Debug, Default, PartialEq)]
#[table_name = "kanji"]
pub struct Kanji {
    pub id: i32,
    pub literal: String,
    pub grade: Option<i32>,
    pub radical: Option<i32>,
    pub stroke_count: i32,
    pub frequency: Option<i32>,
    pub jlpt: Option<i32>,
    pub variant: Option<Vec<String>>,
    pub onyomi: Option<Vec<String>>,
    pub kunyomi: Option<Vec<String>>,
    pub chinese: Option<Vec<String>>,
    pub korean_r: Option<Vec<String>>,
    pub korean_h: Option<Vec<String>>,
    pub natori: Option<Vec<String>>,
    pub kun_dicts: Option<Vec<i32>>,
    pub on_dicts: Option<Vec<i32>>,
}

impl SQL for Kanji {
    fn get_tablename() -> &'static str {
        "kanji"
    }

    fn get_select() -> String {
        "SELECT * FROM kanji".to_string()
    }
}

impl FromRow for Kanji {
    fn from_row(row: &Row, offset: usize) -> Self {
        Kanji {
            id: row.get(offset + 0),
            literal: row.get(offset + 1),
            grade: row.get(offset + 2),
            radical: row.get(offset + 3),
            stroke_count: row.get(offset + 4),
            frequency: row.get(offset + 5),
            jlpt: row.get(offset + 6),
            variant: row.get(offset + 7),
            onyomi: row.get(offset + 8),
            kunyomi: row.get(offset + 9),
            chinese: row.get(offset + 10),
            korean_r: row.get(offset + 11),
            korean_h: row.get(offset + 12),
            natori: row.get(offset + 13),
            kun_dicts: row.get(offset + 14),
            on_dicts: row.get(offset + 15),
        }
    }
}

#[derive(Insertable, Clone, Debug, PartialEq, Default)]
#[table_name = "kanji"]
pub struct NewKanji {
    pub literal: String,
    pub grade: Option<i32>,
    pub radical: Option<i32>,
    pub stroke_count: i32,
    pub frequency: Option<i32>,
    pub jlpt: Option<i32>,
    pub variant: Option<Vec<String>>,
    pub onyomi: Option<Vec<String>>,
    pub kunyomi: Option<Vec<String>>,
    pub chinese: Option<Vec<String>>,
    pub korean_r: Option<Vec<String>>,
    pub korean_h: Option<Vec<String>>,
    pub natori: Option<Vec<String>>,
    pub kun_dicts: Option<Vec<i32>>,
    pub on_dicts: Option<Vec<i32>>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KanjiResult {
    pub kanji: Kanji,
    pub meanings: Vec<String>,
}

impl SQL for KanjiResult {
    fn get_tablename() -> &'static str {
        // It is not a table
        ""
    }

    fn get_select() -> String {
        "SELECT kanji.*, kanji_meaning.* FROM kanji JOIN kanji_meaning ON kanji_meaning.kanji_id = kanji.id".to_string()
    }
}

impl FromRows for KanjiResult {
    fn from_rows(rows: Vec<Row>, _offset: usize) -> Vec<Self>
    where
        Self: Sized,
    {
        format_results(
            rows.into_iter()
                .map(|i| (Kanji::from_row(&i, 0), Meaning::from_row(&i, 16)))
                .collect(),
        )
    }
}

#[derive(Queryable, QueryableByName, Clone, Debug, Default, PartialEq)]
#[table_name = "kanji_element"]
pub struct KanjiElement {
    pub id: i32,
    pub kanji_id: i32,
    pub search_radical_id: i32,
}

impl SQL for KanjiElement {
    fn get_tablename() -> &'static str {
        "kanji_element"
    }
}

impl FromRow for KanjiElement {
    fn from_row(row: &Row, offset: usize) -> Self
    where
        Self: Sized,
    {
        Self {
            id: row.get(offset + 0),
            kanji_id: row.get(offset + 1),
            search_radical_id: row.get(offset + 2),
        }
    }
}

#[derive(Insertable, Clone, Debug, Default, PartialEq)]
#[table_name = "kanji_element"]
pub struct NewKanjiElement {
    pub kanji_id: i32,
    pub search_radical_id: i32,
}

impl From<Character> for NewKanji {
    fn from(k: Character) -> Self {
        Self {
            literal: k.literal.into(),
            grade: k.grade,
            stroke_count: k.stroke_count,
            frequency: k.frequency,
            jlpt: k.jlpt,
            variant: to_option(k.variant),
            onyomi: to_option(k.on_readings),
            kunyomi: to_option(k.kun_readings),
            chinese: to_option(k.chinese_readings),
            korean_r: to_option(k.korean_romanized),
            korean_h: to_option(k.korean_hangul),
            natori: to_option(k.natori),
            kun_dicts: None,
            on_dicts: None,
            radical: k.radical,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadingType {
    Kunyomi,
    Onyomi,
}

impl Kanji {
    /// Returns all dict entries assigned to the kanji's kun readings
    pub async fn get_kun_readings(db: &DbConnection, ids: &[i32]) -> Result<Vec<Dict>, Error> {
        dict::load_by_ids(db, ids).await
    }

    pub async fn load_radical(&self, db: &Pool) -> Result<Option<Radical>, Error> {
        Ok(radical::find_by_id(db, self.radical.unwrap()).await?)
    }

    // TODO replace with gettext string
    /// Print kanji grade pretty for frontend
    pub fn school_str(&self) -> Option<String> {
        self.grade.map(|grade| format!("Taught in {} grade", grade))
    }

    /// Returns the ReadingType of the reading for a kanji
    pub fn get_reading_type(&self, reading: &String) -> Option<ReadingType> {
        let in_on = self.in_on_reading(reading);
        let in_kun = self.in_kun_reading(reading);

        if in_on && !in_kun {
            return Some(ReadingType::Onyomi);
        } else if !in_on && in_kun {
            return Some(ReadingType::Kunyomi);
        }

        None
    }

    pub fn in_kun_reading(&self, reading: &String) -> bool {
        self.kunyomi
            .as_ref()
            .and_then(|j| j.contains(reading).then(|| true))
            .unwrap_or(false)
    }

    pub fn in_on_reading(&self, reading: &String) -> bool {
        self.onyomi
            .as_ref()
            .and_then(|j| j.contains(reading).then(|| true))
            .unwrap_or(false)
    }

    /// Returns true if kanji has a given reading
    pub fn has_reading(&self, reading: &String) -> bool {
        self.kunyomi
            .as_ref()
            .and_then(|i| i.contains(reading).then(|| true))
            .unwrap_or_else(|| {
                self.onyomi
                    .as_ref()
                    .and_then(|j| j.contains(reading).then(|| true))
                    .unwrap_or(false)
            })
    }

    /// Find sequence ids of dict entries containing the kanji and the passed reading
    pub async fn find_readings(
        &self,
        db: &Pool,
        reading: &KanjiReading,
        r_type: ReadingType,
        mode: SearchMode,
        only_main_readins: bool,
    ) -> Result<Vec<i32>, Error> {
        find_readings_by_liteal(
            &self.literal,
            db,
            reading.to_owned(),
            r_type,
            mode,
            only_main_readins,
        )
        .await
    }

    pub fn format_reading(&self, reading: &str, r_type: ReadingType) -> String {
        format_reading_with_literal(&self.literal, reading, r_type)
    }

    /// Returns a Vec of the kanjis parts
    pub async fn load_parts(&self, pool: &Pool) -> Result<Vec<String>, Error> {
        let db = pool.get().await?;

        let sql_query = "SELECT literal FROM kanji_element JOIN search_radical ON search_radical.id = kanji_element.search_radical_id WHERE kanji_id = $1";
        let prepared = db.prepare_cached(&sql_query).await?;

        Ok(db
            .query(&prepared, &[&self.id])
            .await?
            .into_iter()
            .map(|i| i.get(0))
            .collect())
    }
}

pub fn format_reading_with_literal(literal: &str, reading: &str, r_type: ReadingType) -> String {
    match r_type {
        ReadingType::Kunyomi => {
            let r = if reading.contains('.') {
                let right = reading.split('.').nth(1).unwrap_or_default();
                format!("{}{}", literal, right)
            } else {
                literal.to_string()
            };
            r.replace("-", "")
        }
        ReadingType::Onyomi => literal.to_string(),
    }
}

/// Find sequence ids of dict entries containing the kanji and the passed reading
pub async fn find_readings_by_liteal(
    literal: &str,
    db: &Pool,
    reading: KanjiReading,
    r_type: ReadingType,
    mode: SearchMode,
    only_main_readins: bool,
) -> Result<Vec<i32>, Error> {
    let query = include_str!("../../sql/words_with_kanji_readings.sql");

    let lit_sql_hg = mode.to_like(format_reading(&reading.reading).to_hiragana());
    let lit_sql_kk = mode.to_like(format_reading(&reading.reading).to_katakana());
    let lit_reading = mode.to_like(format_reading_with_literal(
        literal,
        &reading.reading,
        r_type,
    ));

    let res: Vec<i32> = prepared_query(
        db,
        query,
        &[&lit_sql_hg, &lit_sql_kk, &lit_reading, &only_main_readins],
    )
    .await?;

    Ok(res)
}

pub async fn insert_kanji_part(db: &DbConnection, element: KanjiPart) -> Result<(), Error> {
    // Find kanji
    let kanji = match find_by_literal(db, element.kanji.to_string()).await {
        Ok(v) => v.ok_or(Error::NotFound)?,
        Err(err) => match err {
            Error::DbError(diesel::result::Error::NotFound) => return Ok(()),
            _ => return Err(err),
        },
    };

    // Find search_radicals IDs for all parts
    let literals = try_join_all(
        element
            .radicals
            .into_iter()
            .map(|i| radical::search_radical_find_by_literal(db, i)),
    )
    .await?;

    let new_kanji_elements = literals
        .into_iter()
        .map(|i| NewKanjiElement {
            search_radical_id: i.id,
            kanji_id: kanji.kanji.id,
        })
        .collect_vec();

    // Insert all search_radicals assigned to the kanji in kanji_elements table
    insert_kanji_elements(db, &new_kanji_elements).await?;

    Ok(())
}

async fn insert_kanji_elements(db: &DbConnection, items: &[NewKanjiElement]) -> Result<(), Error> {
    diesel::insert_into(kanji_element::table)
        .values(items)
        .execute(db)?;
    Ok(())
}

/// Formats a kun/on reading to a kana entry
pub fn format_reading(reading: &str) -> String {
    reading.replace('-', "").replace('.', "")
}

/// Update the jlpt information for a kanji by its literal
pub async fn update_jlpt(db: &DbConnection, l: &str, level: i32) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;
    diesel::update(kanji)
        .filter(literal.eq(l))
        .set(jlpt.eq(level))
        .execute(db)?;
    Ok(())
}

/// Inserts a new kanji into db
pub async fn insert(
    db: &DbConnection,
    kanji_chars: Vec<kanjidict::Character>,
) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;

    let items: Vec<NewKanji> = kanji_chars.iter().map(|i| i.to_owned().into()).collect();

    // Insert kanji into DB
    let kanji_id: Vec<(i32, String)> = diesel::insert_into(kanji)
        .values(items)
        .returning((id, literal))
        .get_results(db)?;

    let new_meanings = kanji_id
        .into_iter()
        .filter_map(|(kid, liter)| {
            let curr_kanji = kanji_chars
                .iter()
                .find(|i| i.literal.to_string() == liter)?;
            Some(curr_kanji.meaning.iter().map(move |j| NewMeaning {
                kanji_id: kid,
                value: j.to_owned(),
            }))
        })
        .flatten()
        .collect_vec();

    // Insert meanings using returned kanji_id
    meaning::insert_meanings(db, new_meanings).await?;

    Ok(())
}

/// Clear all kanji entries
pub async fn clear_kanji_elements(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::kanji_element::dsl::*;
    diesel::delete(kanji_element).execute(db)?;
    Ok(())
}

/// Clear all kanji entries
pub async fn clear_kanji(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;
    diesel::delete(kanji).execute(db)?;
    Ok(())
}

/// Returns Ok(true) if at least one kanji exists in the Db
pub async fn exists(db: &DbConnection) -> Result<bool, Error> {
    use crate::schema::kanji::dsl::*;
    Ok(kanji.select(id).limit(1).execute(db)? == 1)
}

/// Returns Ok(true) if at least one kanji element exists
pub async fn element_exists(db: &DbConnection) -> Result<bool, Error> {
    use crate::schema::kanji_element::dsl::*;
    Ok(kanji_element.select(id).limit(1).execute(db)? == 1)
}

/// Find a kanji by its literal
pub async fn find_by_literalv2(db: &Pool, l: String) -> Result<Option<KanjiResult>, Error> {
    // Try to find literal in kanji cache
    let mut k_cache: MutexGuard<SharedCache<i32, KanjiResult>> = KANJI_RESULT_CACHE.lock().await;
    if let Some(k) = k_cache.find_by_predicate(|i| i.kanji.literal == l) {
        return Ok(Some(k.clone()));
    }

    let db_kanji_res = load_by_literalv2(db, &l).await.map_err(|i| match i {
        Error::DbError(db) => match db {
            diesel::result::Error::NotFound => Error::NotFound,
            _ => Error::DbError(db),
        },
        _ => i,
    });

    let db_kanji = match db_kanji_res {
        Ok(val) => val,
        Err(err) => match err {
            Error::NotFound => return Ok(None),
            _ => return Err(err),
        },
    };

    // Add to cache for future usage
    k_cache.cache_set(db_kanji.kanji.id, db_kanji.clone());

    Ok(Some(db_kanji))
}

/// Find a kanji by its literal
pub async fn find_by_literal(db: &DbConnection, l: String) -> Result<Option<KanjiResult>, Error> {
    // Try to find literal in kanji cache
    let mut k_cache: MutexGuard<SharedCache<i32, KanjiResult>> = KANJI_RESULT_CACHE.lock().await;
    if let Some(k) = k_cache.find_by_predicate(|i| i.kanji.literal == l) {
        return Ok(Some(k.clone()));
    }

    let db_kanji_res = load_by_literal(db, &l).await.map_err(|i| match i {
        Error::DbError(db) => match db {
            diesel::result::Error::NotFound => Error::NotFound,
            _ => Error::DbError(db),
        },
        _ => i,
    });

    let db_kanji = match db_kanji_res {
        Ok(val) => val,
        Err(err) => match err {
            Error::NotFound => return Ok(None),
            _ => return Err(err),
        },
    };

    // Add to cache for future usage
    k_cache.cache_set(db_kanji.kanji.id, db_kanji.clone());

    Ok(Some(db_kanji))
}

/// Find a kanji by its literal
pub async fn find_by_literals(db: &Pool, l: &[String]) -> Result<Vec<KanjiResult>, Error> {
    if l.is_empty() {
        return Ok(vec![]);
    }

    // Try to find literal in kanji cache
    let mut k_cache: MutexGuard<SharedCache<i32, KanjiResult>> = KANJI_RESULT_CACHE.lock().await;

    // Get cached kanji
    let cached_kanji = k_cache.filter_values(|i| l.contains(&i.kanji.literal));

    // Filter all literals which are not cached
    let missing_literals = l
        .iter()
        .filter_map(|i| (!cached_kanji.iter().any(|j| j.kanji.literal == **i)).then(|| i))
        .collect_vec();

    if missing_literals.is_empty() {
        return Ok(cached_kanji);
    }

    let db_kanji = load_by_literals(db, &missing_literals).await?;

    // Add to cache for future usage
    k_cache.extend(db_kanji.clone(), |i| i.kanji.id);

    Ok(cached_kanji.into_iter().chain(db_kanji).collect_vec())
}

/// Find Kanji items by its ids
pub async fn load_by_idsv2(db: &Pool, ids: &[i32]) -> Result<Vec<KanjiResult>, Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    // Lock cache
    let mut k_cache: MutexGuard<SharedCache<i32, KanjiResult>> = KANJI_RESULT_CACHE.lock().await;

    // Get cached kanji
    let cached_kanji = k_cache.get_values(&ids);

    // Determine which of the kanji
    // still needs to get looked up
    let to_lookup = ids
        .iter()
        .filter(|k_id| !cached_kanji.iter().any(|ci| ci.kanji.id == **k_id))
        .copied()
        .collect::<Vec<_>>();

    if to_lookup.is_empty() {
        return Ok(cached_kanji);
    }

    let db_result = retrieve_by_ids_with_meaningsv2(&db, &to_lookup).await?;

    // Add result to cache for next time
    k_cache.extend(db_result.clone(), |i| i.kanji.id);

    Ok([db_result, cached_kanji].concat())
}

/// Find Kanji items by its ids
pub async fn load_by_ids(db: &DbConnection, ids: &[i32]) -> Result<Vec<KanjiResult>, Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    // Lock cache
    let mut k_cache: MutexGuard<SharedCache<i32, KanjiResult>> = KANJI_RESULT_CACHE.lock().await;

    // Get cached kanji
    let cached_kanji = k_cache.get_values(&ids);

    // Determine which of the kanji
    // still needs to get looked up
    let to_lookup = ids
        .iter()
        .filter(|k_id| !cached_kanji.iter().any(|ci| ci.kanji.id == **k_id))
        .copied()
        .collect::<Vec<_>>();

    if to_lookup.is_empty() {
        return Ok(cached_kanji);
    }

    let db_result = retrieve_by_ids_with_meanings(&db, &to_lookup).await?;

    // Add result to cache for next time
    k_cache.extend(db_result.clone(), |i| i.kanji.id);

    Ok([db_result, cached_kanji].concat())
}

/// Retrieve kanji by ids from DB
async fn retrieve_by_ids_with_meaningsv2(
    db: &Pool,
    ids: &[i32],
) -> Result<Vec<KanjiResult>, Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }

    let sql_query = KanjiResult::select_where("kanji.id = ANY($1)");
    Ok(KanjiResult::query(db, sql_query, &[&ids], 0).await?)
}

/// Retrieve kanji by ids from DB
async fn retrieve_by_ids_with_meanings(
    db: &DbConnection,
    ids: &[i32],
) -> Result<Vec<KanjiResult>, Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }

    use crate::schema::kanji::dsl::*;
    use crate::schema::kanji_meaning;

    Ok(format_results(
        kanji
            .inner_join(kanji_meaning::table)
            .filter(id.eq_any(ids))
            .get_results::<(Kanji, Meaning)>(db)?,
    ))
}

/// Load a kanji by its literal from DB
async fn load_by_literals(db: &Pool, l: &[&String]) -> Result<Vec<KanjiResult>, Error> {
    if l.is_empty() {
        return Ok(vec![]);
    }

    let sql_query = KanjiResult::select_where("literal = ANY($1)");
    Ok(KanjiResult::query(&db, sql_query, &[&l], 0).await?)
}

/// Load a kanji by its literal from DB
async fn load_by_literalv2(db: &Pool, l: &str) -> Result<KanjiResult, Error> {
    let sql = KanjiResult::select_where_limit("literal = $1", 1);
    let mut res = KanjiResult::query(db, sql, &[&l], 0).await?;

    if res.is_empty() {
        return Err(Error::NotFound);
    }

    Ok(take(&mut res[0]))
}

/// Load a kanji by its literal from DB
async fn load_by_literal(db: &DbConnection, l: &str) -> Result<KanjiResult, Error> {
    use crate::schema::kanji::dsl::*;
    use crate::schema::kanji_meaning;

    let res = kanji
        .inner_join(kanji_meaning::table)
        .filter(literal.eq(l))
        .limit(1)
        .get_results::<(Kanji, Meaning)>(db)?;

    if res.is_empty() {
        return Err(Error::NotFound);
    }

    Ok(format_results(res)[0].to_owned())
}

fn format_results(res: Vec<(Kanji, Meaning)>) -> Vec<KanjiResult> {
    res.into_iter()
        .group_by(|i| i.0.id)
        .into_iter()
        .map(|(_, e)| {
            let mut kanji = e.collect_vec().into_iter().peekable();

            KanjiResult {
                kanji: kanji.peek().map(|i| &i.0).unwrap().clone(),
                meanings: kanji.map(|i| i.1.value).collect_vec(),
            }
        })
        .collect_vec()
}

/// An in memory Cache for kanji items
pub static KANJICACHE: Lazy<
    std::sync::Mutex<SharedCache<String, (Option<Vec<String>>, Option<Vec<String>>)>>,
> = Lazy::new(|| std::sync::Mutex::new(SharedCache::with_capacity(10000)));

pub async fn load_kanji_cache(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;

    let all_kanji: Vec<(String, Option<Vec<String>>, Option<Vec<String>>)> =
        kanji.select((literal, kunyomi, onyomi)).get_results(db)?;

    let mut kanji_cache = KANJICACHE.lock().unwrap();
    for curr_kanji in all_kanji {
        kanji_cache.cache_set(curr_kanji.0, (curr_kanji.1, curr_kanji.2));
    }

    Ok(())
}
