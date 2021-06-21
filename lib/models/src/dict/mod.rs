pub mod collocations;

use std::cmp::Ordering;

use super::{
    kanji::{KanjiResult, KANJICACHE},
    sense,
};
use crate::queryable::{
    prepared_execute, prepared_query, prepared_query_one, CheckAvailable, Deletable, FromRow,
    Insertable, SQL, Queryable,
};
use deadpool_postgres::{tokio_postgres::Row, Pool};
use error::Error;
use futures::future::try_join_all;
use itertools::Itertools;
use japanese::{self, furigana, JapaneseExt};
use parse::{
    accents::PitchItem,
    jmdict::{information::Information, languages::Language, priority::Priority, Entry},
};
use tokio_postgres::types::ToSql;

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug, PartialEq)]
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

impl SQL for NewDict {
    fn get_tablename() -> &'static str {
        "dict"
    }
}

impl Insertable<12> for NewDict {
    fn column_names() -> [&'static str; 12] {
        [
            "sequence",
            "reading",
            "kanji",
            "no_kanji",
            "priorities",
            "information",
            "kanji_info",
            "jlpt_lvl",
            "is_main",
            "accents",
            "furigana",
            "collocations",
        ]
    }
    fn fields(&self) -> [&(dyn ToSql + Sync); 12] {
        [
            &self.sequence,
            &self.reading,
            &self.kanji,
            &self.no_kanji,
            &self.priorities,
            &self.information,
            &self.kanji_info,
            &self.jlpt_lvl,
            &self.is_main,
            &self.accents,
            &self.furigana,
            &self.collocations,
        ]
    }
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

pub async fn update_accents(
    db: &Pool,
    pitch: impl Iterator<Item = PitchItem>,
) -> Result<(), Error> {
    try_join_all(pitch.map(|i| update_accent(db, i))).await?;
    Ok(())
}

async fn update_accent(db: &Pool, pitch: PitchItem) -> Result<(), Error> {
    let seq = find_jp_word(db, &pitch.kanji, &pitch.kana).await?;
    if seq.is_none() {
        return Ok(());
    }
    let seq = seq.unwrap();

    prepared_execute(
        db,
        "UPDATE dict SET accents=$1 WHERE sequence=$2 AND reading = $3",
        &[&pitch.pitch, &seq, &pitch.kana],
    )
    .await?;
    Ok(())
}

pub async fn find_jp_word(db: &Pool, kanji: &str, kana: &str) -> Result<Option<i32>, Error> {
    let query = include_str!("../../sql/find_jp_word.sql");

    let sequence: Vec<i32> = prepared_query(db, query, &[&kanji, &kana]).await?;

    if sequence.is_empty() {
        return Ok(None);
    }

    Ok(Some(sequence[0]))
}

pub async fn update_jlpt(db: &Pool, l: &str, level: i32) -> Result<(), Error> {
    let query = "UPDATE dict SET JLpt_lvl=$1 WHERE sequence = ANY(SELECT sequence FROM dict WHERE reading = $2)";
    prepared_execute(db, query, &[&level, &l]).await?;
    Ok(())
}

/// Get all Database-dict structures from an entry
pub fn new_dicts_from_entry(entry: &Entry) -> Vec<NewDict> {
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
                furigana::generate::checked(|l: String| get_kanji(&l), &main.reading, &kana);
            main.furigana = Some(furigana);
        }
    }

    dicts
}

fn get_kanji(l: &str) -> Option<(Option<Vec<String>>, Option<Vec<String>>)> {
    let lock = KANJICACHE.lock().unwrap();
    lock.cache_get(&l.to_owned()).map(|i| i.to_owned())

    /*
    let db = db.get().await.unwrap();
    let sql = "SELECT kunyomi, onyomi FROM kanji WHERE literal=$1";
    let prepared = db.prepare_cached(sql).await.unwrap();
    let row = db.query_opt(&prepared, &[&l]).await.ok()??;
    let readings: (Option<Vec<String>>, Option<Vec<String>>) = (row.get(0), row.get(1));

    lock.cache_set(l.to_owned(), readings.clone());
    */

    //Some(readings)
}

/*
pub async fn load_by_ids(db: &DbConnection, ids: &[i32]) -> Result<Vec<Dict>, Error> {
    use crate::schema::dict::dsl::*;
    if ids.is_empty() {
        return Ok(vec![]);
    }
    Ok(dict.filter(id.eq_any(ids)).get_results(db)?)
}
*/

/// Finds words by their exact readings and retuns a vec of their sequence ids
#[cfg(feature = "tokenizer")]
pub(crate) async fn find_by_reading(
    db: &Pool,
    readings: &[(&str, i32, bool)],
) -> Result<Vec<(i32, i32)>, Error> {
    if readings.is_empty() {
        return Ok(vec![]);
    }

    let mut result = Vec::new();
    for (reading_str, start, only_kana) in readings {
        /*
        let dict_res: Result<Vec<Dict>, _> = if *only_kana {
            dict.filter(reading.eq(reading_str))
                .filter(is_main.eq(true))
                .get_results(db)
        } else {
            dict.filter(reading.eq(reading_str)).get_results(db)
        };
        */

        let mut dict_res: Vec<Dict> = if *only_kana {
            Dict::query(
                db,
                Dict::select_where("reading = $1 AND is_main = true"),
                &[&reading_str],
                0,
            )
            .await?
        } else {
            Dict::query(db, Dict::select_where("reading = $1"), &[&reading_str], 0).await?
        };

        if dict_res.is_empty() {
            continue;
        }

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

/// Returns Ok(true) if at least one dict exists in the Db
pub async fn exists(db: &Pool) -> Result<bool, Error> {
    Dict::exists(db).await
}

/// Insert multiple dicts into the database
pub async fn insert_dicts(db: &Pool, dicts: Vec<NewDict>) -> Result<(), Error> {
    NewDict::insert(db, &dicts).await?;
    Ok(())
}

/// Clear all dict entries
pub async fn clear_dicts(db: &Pool) -> Result<(), Error> {
    Dict::delete_all(db).await?;
    Ok(())
}

/// Get the min(sequence) of all dicts
pub async fn min_sequence(db: &Pool) -> Result<i32, Error> {
    Ok(prepared_query_one(db, "SELECT MIN(sequence) FROM dict", &[]).await?)
}

/// Load Dictionaries of a single sequence id
pub async fn load_dictionary(db: &Pool, sequence_id: i32) -> Result<Vec<Dict>, Error> {
    let sql = Dict::select_where_order("sequence=$1", "id");
    let res = prepared_query(db, sql, &[&sequence_id]).await?;

    Ok(res)

    /*
    use crate::schema::dict as dict_schema;
    Ok(dict_schema::table
        .filter(dict_schema::sequence.eq_all(sequence_id))
        .order_by(dict_schema::id)
        .get_results(db)?)
        */
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
