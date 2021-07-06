pub mod gen_readings;
pub mod meaning;
pub mod reading;

use std::mem::take;

use deadpool_postgres::{tokio_postgres::Row, Pool};
use reading::KanjiReading;
use tokio_postgres::types::ToSql;

use crate::queryable::{
    self, prepared_execute, prepared_query, CheckAvailable, Deletable, FromRow, FromRows,
    Insertable, Queryable, SQL,
};

use super::{
    dict,
    kanji::meaning::{Meaning, NewMeaning},
    radical::{self, Radical},
};
use crate::search_mode::SearchMode;
use cache::SharedCache;
use error::Error;
use parse::{
    kanji_ele::KanjiPart,
    kanjidict::{self, Character},
};
use utils::to_option;

use async_std::sync::{Mutex, MutexGuard};
use futures::future::try_join_all;
use itertools::Itertools;
use once_cell::sync::Lazy;
use romaji::RomajiExt;

/// An in memory Cache for kanji items
static KANJI_RESULT_CACHE: Lazy<Mutex<SharedCache<i32, KanjiResult>>> =
    Lazy::new(|| Mutex::new(SharedCache::with_capacity(10000)));

#[derive(Clone, Debug, Default, PartialEq)]
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
    pub similar_kanji: Option<Vec<String>>,
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
            similar_kanji: row.get(offset + 16),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
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
    pub similar_kanji: Option<Vec<String>>,
}

impl SQL for NewKanji {
    fn get_tablename() -> &'static str {
        "kanji"
    }
}

impl Insertable<16> for NewKanji {
    fn column_names() -> [&'static str; 16] {
        [
            "literal",
            "grade",
            "radical",
            "stroke_count",
            "frequency",
            "jlpt",
            "variant",
            "onyomi",
            "kunyomi",
            "chinese",
            "korean_r",
            "korean_h",
            "natori",
            "kun_dicts",
            "on_dicts",
            "similar_kanji",
        ]
    }

    fn fields(&self) -> [&(dyn ToSql + Sync); 16] {
        [
            &self.literal,
            &self.grade,
            &self.radical,
            &self.stroke_count,
            &self.frequency,
            &self.jlpt,
            &self.variant,
            &self.onyomi,
            &self.kunyomi,
            &self.chinese,
            &self.korean_r,
            &self.korean_h,
            &self.natori,
            &self.kun_dicts,
            &self.on_dicts,
            &self.similar_kanji,
        ]
    }
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
                .map(|i| (Kanji::from_row(&i, 0), Meaning::from_row(&i, 17)))
                .collect(),
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NewKanjiElement {
    pub kanji_id: i32,
    pub search_radical_id: i32,
}

impl SQL for NewKanjiElement {
    fn get_tablename() -> &'static str {
        "kanji_element"
    }
}

impl queryable::Insertable<2> for NewKanjiElement {
    fn fields(&self) -> [&(dyn ToSql + Sync); 2] {
        [&self.kanji_id, &self.search_radical_id]
    }

    fn column_names() -> [&'static str; 2] {
        ["kanji_id", "search_radical_id"]
    }
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
            similar_kanji: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadingType {
    Kunyomi,
    Onyomi,
}

impl Kanji {
    /*
    /// Returns all dict entries assigned to the kanji's kun readings
    pub async fn get_kun_readings(db: &DbConnection, ids: &[i32]) -> Result<Vec<Dict>, Error> {
        dict::load_by_ids(db, ids).await
    }
    */

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

    pub fn get_stroke_frames_url(&self) -> String {
        format!("/assets/svg/{}_frames.svg", self.literal)
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

pub async fn insert_kanji_parts(db: &Pool, elements: &[KanjiPart]) -> Result<(), Error> {
    try_join_all(elements.into_iter().map(|i| insert_kanji_part(db, i))).await?;
    Ok(())
}

async fn insert_kanji_part(db: &Pool, element: &KanjiPart) -> Result<(), Error> {
    // Find kanji
    let kanji = match find_by_literalv2(db, element.kanji.to_string()).await? {
        Some(v) => v,
        None => return Ok(()),
    };

    // Find search_radicals IDs for all parts
    let literals = try_join_all(
        element
            .radicals
            .iter()
            .map(|i| radical::search_radical_find_by_literal(db, *i)),
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

async fn insert_kanji_elements(db: &Pool, items: &[NewKanjiElement]) -> Result<(), Error> {
    NewKanjiElement::insert(db, items).await?;
    Ok(())
}

/// Formats a kun/on reading to a kana entry
pub fn format_reading(reading: &str) -> String {
    reading.replace('-', "").replace('.', "")
}

/// Update the jlpt information for a kanji by its literal
pub async fn update_jlpt(db: &Pool, l: &str, level: i32) -> Result<(), Error> {
    let query = "UPDATE kanji SET jlpt=$1 WHERE literal=$2";
    prepared_execute(db, query, &[&level, &l]).await?;
    Ok(())
}

/// Inserts a new kanji into db
pub async fn insert(db: &Pool, kanji_chars: Vec<kanjidict::Character>) -> Result<(), Error> {
    if kanji_chars.is_empty() {
        return Ok(());
    }

    let items: Vec<NewKanji> = kanji_chars.iter().map(|i| i.to_owned().into()).collect();

    let query = NewKanji::get_insert_query(items.len());
    let query = format!("{} RETURNING id, literal", query);

    let kanji_id: Vec<(i32, String)> =
        prepared_query(db, query, &NewKanji::get_bind_data(&items)).await?;

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
pub async fn clear_kanji_elements(db: &Pool) -> Result<(), Error> {
    KanjiElement::delete_all(db).await?;
    Ok(())
}

/// Clear all kanji entries
pub async fn clear_kanji(db: &Pool) -> Result<(), Error> {
    Kanji::delete_all(db).await?;
    Ok(())
}

/// Returns Ok(true) if at least one kanji exists in the Db
pub async fn exists(db: &Pool) -> Result<bool, Error> {
    Kanji::exists(db).await
}

/// Returns Ok(true) if at least one kanji element exists
pub async fn element_exists(db: &Pool) -> Result<bool, Error> {
    Kanji::exists(db).await
}

/// Find a kanji by its literal
pub async fn find_by_literalv2(db: &Pool, l: String) -> Result<Option<KanjiResult>, Error> {
    // Try to find literal in kanji cache
    let mut k_cache: MutexGuard<SharedCache<i32, KanjiResult>> = KANJI_RESULT_CACHE.lock().await;
    if let Some(k) = k_cache.find_by_predicate(|i| i.kanji.literal == l) {
        return Ok(Some(k.clone()));
    }

    let db_kanji = match load_by_literalv2(db, &l).await? {
        Some(v) => v,
        None => return Ok(None),
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

/// Load a kanji by its literal from DB
async fn load_by_literals(db: &Pool, l: &[&String]) -> Result<Vec<KanjiResult>, Error> {
    if l.is_empty() {
        return Ok(vec![]);
    }

    let sql_query = KanjiResult::select_where("literal = ANY($1)");
    Ok(KanjiResult::query(&db, sql_query, &[&l], 0).await?)
}

/// Load a kanji by its literal from DB
async fn load_by_literalv2(db: &Pool, l: &str) -> Result<Option<KanjiResult>, Error> {
    let sql = KanjiResult::select_where_limit("literal = $1", 1);
    let mut res = KanjiResult::query(db, sql, &[&l], 0).await?;

    if res.is_empty() {
        return Ok(None);
    }

    Ok(Some(take(&mut res[0])))
}

/// Updates similar kanji of a kanji by its literal
pub async fn set_similarkanji(db: &Pool, literal: char, similar: &[char]) -> Result<(), Error> {
    let literal = literal.to_string();
    let similar: Vec<_> = similar.into_iter().map(|i| i.to_string()).collect();
    let query = "UPDATE kanji SET similar_kanji=$1 WHERE literal=$2";
    prepared_execute(db, query, &[&similar, &literal]).await?;
    Ok(())
}

/// Resets all similar kanji
pub async fn clear_similar_kanji(db: &Pool) -> Result<(), Error> {
    prepared_execute(db, "UPDATE kanji SET similar_kanji=NULL", &[]).await?;
    Ok(())
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

pub async fn load_kanji_cache(db: &Pool) -> Result<(), Error> {
    let db = db.get().await?;
    let prepared = db
        .prepare("SELECT literal, kunyomi, onyomi FROM kanji")
        .await?;
    let res = db.query(&prepared, &[]).await?;
    let all_kanji: Vec<(String, Option<Vec<String>>, Option<Vec<String>>)> = res
        .into_iter()
        .map(|i| (i.get(0), i.get(1), i.get(2)))
        .collect();

    let mut kanji_cache = KANJICACHE.lock().unwrap();
    for curr_kanji in all_kanji {
        kanji_cache.cache_set(curr_kanji.0, (curr_kanji.1, curr_kanji.2));
    }

    Ok(())
}
