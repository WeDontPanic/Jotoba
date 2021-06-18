use std::{
    cmp::Ordering,
    io::{stdout, Write},
};

use super::dict::Dict;
use super::ReadingType;

use crate::{search_mode::SearchMode, DbConnection};
use error::Error;
use japanese::JapaneseExt;
use utils;

use diesel::prelude::*;
use futures::future::try_join_all;
use itertools::Itertools;

type KanjiWReading = (i32, String, Vec<String>);

/// Update kun/on reading compounds
pub async fn update_links(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;

    clear_links(db).await?;

    let all_kanji: Vec<(i32, String, Option<Vec<String>>, Option<Vec<String>>)> = kanji
        .select((id, literal, kunyomi, onyomi))
        .get_results(db)?;

    let all_kuns = all_kanji
        .iter()
        .filter_map(|i| {
            (i.2.is_some() && !i.2.as_ref().unwrap().is_empty())
                .then(|| (i.0, i.1.to_owned(), i.2.as_ref().unwrap().to_owned()))
        })
        .collect::<Vec<KanjiWReading>>();

    gen_reading(db, all_kuns, ReadingType::Kunyomi).await?;

    let all_ons = all_kanji
        .iter()
        .filter_map(|i| {
            (i.3.is_some() && !i.3.as_ref().unwrap().is_empty())
                .then(|| (i.0, i.1.to_owned(), i.3.as_ref().unwrap().to_owned()))
        })
        .collect::<Vec<KanjiWReading>>();

    gen_reading(db, all_ons, ReadingType::Onyomi).await?;

    Ok(())
}

// Generate and store readingcompounds of kanji
async fn gen_reading(
    db: &DbConnection,
    kanji: Vec<KanjiWReading>,
    reading_type: ReadingType,
) -> Result<(), Error> {
    let mut counter = 0;

    for kanji_item_chunk in kanji.chunks(100).into_iter() {
        print!(
            "\r Generating {:?} {}%",
            reading_type,
            counter * 100 / kanji.len()
        );
        stdout().flush().ok();

        let curr_item_count = kanji_item_chunk.len();

        let res: Vec<(i32, Vec<i32>)> = try_join_all(
            kanji_item_chunk
                .into_iter()
                .map(|chunk| find_readings(db, chunk.1.clone(), &chunk.2, reading_type, chunk.0)),
        )
        .await?;

        futures::future::try_join_all(
            res.iter()
                .map(|(k_id, dict_ids)| update_link(db, reading_type, *k_id, dict_ids)),
        )
        .await?;

        counter += curr_item_count;
    }

    println!();

    Ok(())
}

/// Returns all kun reading compounds for a kanji given by its literal
async fn find_readings(
    db: &DbConnection,
    literal: String,
    readings: &[String], // All kanji kun readings
    reading_type: ReadingType,
    kid: i32,
) -> Result<(i32, Vec<i32>), Error> {
    use crate::schema::dict::dsl::*;

    let formatter = match reading_type {
        ReadingType::Kunyomi => format!("{}%", literal),
        ReadingType::Onyomi => format!("%{}%", literal),
    };

    // Find all Dict-seq_ids starting with the literal
    let seq_ids: Vec<i32> = dict
        .select(sequence)
        .filter(reading.like(formatter))
        .filter(kanji.eq(true))
        .get_results(db)?;

    let dicts: Vec<Dict> = dict
        .filter(sequence.eq_any(seq_ids))
        .order_by(id)
        .get_results(db)?;

    // result vec
    let mut compound_dicts: Vec<Dict> = Vec::new();

    // Iterate over all dicts containing the literal
    for (_, val) in dicts.iter().group_by(|i| i.sequence).into_iter() {
        let (kanji_r, kana_r): (Vec<Dict>, Vec<Dict>) =
            val.into_iter().cloned().partition(|i| i.kanji);

        if kanji_r.is_empty() {
            continue;
        }

        // kana reading of curr dict
        let dict_kana = kana_r[0].clone();
        // kanji reading of curr dict
        let dict_kanji = kanji_r[0].clone();

        for curr_reading in readings.iter() {
            if matches_kanji(
                &literal,
                reading_type,
                curr_reading,
                &dict_kana.reading,
                &dict_kanji.reading,
            ) && len(curr_reading) <= dict_kana.len()
            {
                compound_dicts.push(dict_kanji);
                break;
            }
        }
    }

    match reading_type {
        ReadingType::Kunyomi => {
            let clean_kuns = readings.iter().map(|i| literal_reading(i)).collect_vec();
            compound_dicts.sort_by(|a, b| order_kun(a, b, &clean_kuns))
        }
        ReadingType::Onyomi => compound_dicts.sort_by(|a, b| order_on(a, b, &literal)),
    }
    compound_dicts.truncate(10);

    Ok((kid, compound_dicts.iter().map(|i| i.sequence).collect()))
}

// TODO implement a better order algo
fn order_on(a: &Dict, b: &Dict, literal: &str) -> Ordering {
    if a.reading.contains(literal) && !b.reading.contains(literal) {
        return Ordering::Less;
    } else if b.reading.contains(literal) && !a.reading.contains(literal) {
        return Ordering::Greater;
    }

    let a_sw = a.reading.starts_with(literal);
    let b_sw = b.reading.starts_with(literal);

    if a.is_main && !b.is_main {
        return Ordering::Less;
    } else if b.is_main && !a.is_main {
        return Ordering::Greater;
    }

    if a.reading.len() < b.reading.len() {
        return Ordering::Less;
    } else if a.reading.len() > b.reading.len() {
        return Ordering::Greater;
    }

    if a_sw && !b_sw {
        return Ordering::Less;
    } else if b_sw && !a_sw {
        return Ordering::Greater;
    }

    let a_prio = a.priorities.as_ref().map(|i| i.len()).unwrap_or_default();
    let b_prio = b.priorities.as_ref().map(|i| i.len()).unwrap_or_default();

    if a_prio > b_prio && a_prio > 0 {
        return Ordering::Less;
    } else if b_prio > a_prio && b_prio > 0 {
        return Ordering::Greater;
    }

    Ordering::Equal
}

// TODO implement a better order algo
fn order_kun(a: &Dict, b: &Dict, clean_kuns: &Vec<String>) -> Ordering {
    let a_kunr = clean_kuns.contains(&a.reading);
    let b_kunr = clean_kuns.contains(&b.reading);

    if a_kunr && b_kunr {
        if let Some(order) = utils::get_item_order(&clean_kuns, &a.reading, &b.reading) {
            return order;
        }
    } else if a_kunr && !b_kunr {
        return Ordering::Less;
    } else if !a_kunr && b_kunr {
        return Ordering::Greater;
    }

    if a.is_main && !b.is_main {
        return Ordering::Less;
    } else if b.is_main && !a.is_main {
        return Ordering::Greater;
    }

    /*
    #[cfg(feature = "tokenizer")]
    let a_parsed = JA_NL_PARSER.parse(&a.reading).len();
    #[cfg(feature = "tokenizer")]
    let b_parsed = JA_NL_PARSER.parse(&b.reading).len();
    #[cfg(feature = "tokenizer")]
    if a_parsed == 1 && b_parsed > 0 {
        return Ordering::Less;
    } else if a_parsed > 1 && b_parsed == 0 {
        return Ordering::Greater;
    }
    */

    let a_prio = a.priorities.as_ref().map(|i| i.len()).unwrap_or_default();
    let b_prio = b.priorities.as_ref().map(|i| i.len()).unwrap_or_default();
    if a_prio > 0 && b_prio == 0 {
        return Ordering::Less;
    } else if b_prio > 0 && a_prio == 0 {
        return Ordering::Greater;
    }

    let a_jlpt = a.jlpt_lvl;
    let b_jlpt = b.jlpt_lvl;

    if a_jlpt.is_some() && b_jlpt.is_none() {
        return Ordering::Less;
    } else if a_jlpt.is_none() && b_jlpt.is_some() {
        return Ordering::Greater;
    }

    if a_jlpt.is_some() && b_jlpt.is_some() {
        let a_jlpt = a_jlpt.unwrap();
        let b_jlpt = b_jlpt.unwrap();
        return a_jlpt.cmp(&b_jlpt).reverse();
    }

    Ordering::Equal
}

pub fn len(kun: &str) -> usize {
    utils::real_string_len(&super::format_reading(kun))
}

pub fn literal_reading(kun: &str) -> String {
    kun.replace('-', "").split('.').next().unwrap().to_string()
}

async fn update_link(
    db: &DbConnection,
    reading_type: ReadingType,
    kanji_id: i32,
    dict_ids: &[i32],
) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;

    if dict_ids.is_empty() {
        return Ok(());
    }

    if reading_type == ReadingType::Onyomi {
        diesel::update(kanji)
            .filter(id.eq(kanji_id))
            .set(on_dicts.eq(dict_ids))
            .execute(db)?;
    } else if reading_type == ReadingType::Kunyomi {
        diesel::update(kanji)
            .filter(id.eq(kanji_id))
            .set(kun_dicts.eq(dict_ids))
            .execute(db)?;
    }
    Ok(())
}

fn matches_kanji(
    literal: &str,
    reading_type: ReadingType,
    reading: &str,
    kana_reading: &str,
    kanji_reading: &str,
) -> bool {
    match reading_type {
        ReadingType::Kunyomi => matches_kun(literal, reading, kana_reading, kanji_reading),
        ReadingType::Onyomi => matches_on(literal, reading, kana_reading, kanji_reading),
    }
}

fn matches_kun(literal: &str, reading: &str, kana_reading: &str, kanji_reading: &str) -> bool {
    let match_mode = if reading.starts_with('-') {
        SearchMode::RightVariable
    } else if reading.ends_with('-') || kanji_reading.starts_with(&literal) {
        SearchMode::LeftVariable
    } else {
        SearchMode::Exact
    };

    let mut kanji_out = reading.to_string().replace('-', "");

    if reading.contains('.') {
        let kun_left = reading.split('.').next().unwrap();
        kanji_out = kanji_out.replace(&format!("{}.", kun_left), literal);
    } else {
        kanji_out = literal.to_owned();
    }

    let kanji_out = kanji_out.replace(literal, &literal_reading(reading));
    match_mode.str_eq(kana_reading, kanji_out.as_str(), false)
}

fn matches_on(literal: &str, reading: &str, kana_reading: &str, kanji_reading: &str) -> bool {
    let reading = reading.to_hiragana();
    let kana_reading = kana_reading.to_hiragana();
    if kanji_reading.starts_with(literal) {
        kana_reading.starts_with(&reading)
    } else if kanji_reading.ends_with(literal) {
        kana_reading.ends_with(&reading)
    } else {
        kana_reading.contains(&reading)
            && !kana_reading.starts_with(&reading)
            && !kana_reading.ends_with(&reading)
    }
}

/// Clear existinig kun links
async fn clear_links(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::kanji::dsl::*;

    let empty: Option<Vec<i32>> = None;
    diesel::update(kanji)
        .set((kun_dicts.eq(&empty), on_dicts.eq(&empty)))
        .execute(db)?;

    Ok(())
}
