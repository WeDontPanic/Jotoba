use std::io::{stdout, Write};

use crate::{error::Error, sql::ExpressionMethods, DbPool};

use diesel::prelude::*;
use futures::future::try_join_all;
use tokio_diesel::*;

use super::Dict;

type GenDict = (i32, String);

/// Generate all collocations
pub async fn generate(db: &DbPool) -> Result<(), Error> {
    use crate::schema::dict;

    println!("Clearing old collocations");

    clear(db).await?;

    print!("Starting collocation generation...");
    stdout().flush().ok();

    // Load woards to generate collocations for
    let to_generate_dicts: Vec<GenDict> = dict::table.select((dict::sequence, dict::reading))
        .filter(dict::kanji.eq_all(true))
        .filter(dict::is_main.eq_all(true))
        .filter(dict::reading.regex_match(
        "^[\\x3400-\\x4DB5\\x4E00-\\x9FCB\\xF900-\\xFA6A]*[^(を|の|に|と|が|か|は|も|で|へ|や)]$",
    )).get_results_async(&db).await?;

    let dict_count = to_generate_dicts.len();

    // Generate them chunked wise
    let mut counter = 0;
    for chunk in to_generate_dicts.chunks(200).into_iter() {
        let cl = chunk.len();

        print!(
            "\rGenerating Collocations: {}%         ",
            counter * 100 / dict_count
        );
        stdout().flush().ok();

        try_join_all(chunk.into_iter().map(|i| generate_dict(db, i))).await?;
        counter += cl;
    }

    println!();
    Ok(())
}

async fn generate_dict(db: &DbPool, dict: &GenDict) -> Result<(), Error> {
    use crate::schema::dict;

    let regex_filter = format!("({})[(を|の|に|と|が|か|は|も|で|へ|や)][\\x3400-\\x4DB5\\x4E00-\\x9FCB\\xF900-\\xFA6A]*[^(を|の|に|と|が|か|は|も|で|へ|や)]$", dict.1);

    // Sequence ids of collocation words
    let collocations: Vec<i32> = dict::table
        .select(dict::sequence)
        .distinct()
        .filter(dict::kanji.eq_all(true))
        .filter(dict::reading.like(format!("{}%", dict.1)))
        .filter(dict::reading.regex_match(regex_filter))
        .get_results_async(db)
        .await?;

    let kana = get_kana(db, &dict).await?;

    let collocations = try_join_all(
        collocations
            .into_iter()
            .map(|d| super::load_dictionary(db, d)),
    )
    .await?
    .into_iter()
    .filter_map(|i| collocation_matches(&i, &kana).then(|| i[0].sequence))
    .collect::<Vec<i32>>();

    if collocations.is_empty() {
        return Ok(());
    }

    // Update collocation references for dict
    diesel::update(dict::table)
        .set(dict::collocations.eq_all(collocations))
        .filter(dict::sequence.eq_all(dict.0))
        .filter(dict::is_main.eq_all(true))
        .execute_async(db)
        .await?;

    Ok(())
}

fn collocation_matches(collocation: &Vec<Dict>, kana: &str) -> bool {
    collocation
        .iter()
        .find(|i| !i.kanji && i.reading.contains(kana))
        .map(|_| true)
        .unwrap_or_default()
}

async fn get_kana(db: &DbPool, gd: &GenDict) -> Result<String, Error> {
    use crate::schema::dict::dsl::*;
    let res: String = dict
        .select(reading)
        .filter(sequence.eq_all(gd.0))
        .filter(kanji.eq_all(false))
        .get_result_async(db)
        .await?;
    Ok(res)
}

/// Clear existinig collections
async fn clear(db: &DbPool) -> Result<(), Error> {
    use crate::schema::dict::dsl::*;

    let empty: Option<Vec<i32>> = None;
    diesel::update(dict)
        .set(collocations.eq_all(&empty))
        .execute_async(&db)
        .await?;

    Ok(())
}
