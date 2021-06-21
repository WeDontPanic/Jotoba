use std::io::{stdout, Write};

use crate::queryable::{prepared_execute, prepared_query, prepared_query_one};
use deadpool_postgres::Pool;
use error::Error;

use futures::future::try_join_all;

use super::Dict;

type GenDict = (i32, String);

/// Generate all collocations
pub async fn generate(db: &Pool) -> Result<(), Error> {
    println!("Clearing old collocations");

    clear(db).await?;

    print!("Starting collocation generation...");
    stdout().flush().ok();

    let sql = "SELECT sequence, reading FROM dict WHERE kanji = true AND is_main = true AND reading ~ '^[\\x3400-\\x4DB5\\x4E00-\\x9FCB\\xF900-\\xFA6A]*[^(を|の|に|と|が|か|は|も|で|へ|や)]$'";
    let to_generate_dicts: Vec<GenDict> = prepared_query(db, sql, &[]).await?;

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

async fn generate_dict(db: &Pool, dict: &GenDict) -> Result<(), Error> {
    let collocations_sql = format!("SELECT DISTINCT sequence FROM dict 
                                   WHERE kanji=true AND reading LIKE $1 AND reading ~ '({})[(を|の|に|と|が|か|は|も|で|へ|や)][\\x3400-\\x4DB5\\x4E00-\\x9FCB\\xF900-\\xFA6A]*[^(を|の|に|と|が|か|は|も|で|へ|や)]$'", dict.1);

    let collocations: Vec<i32> =
        prepared_query(db, collocations_sql, &[&format!("{}%", dict.1)]).await?;

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
    let sql = "UPDATE dict SET collocations=$1 WHERE sequence=$2 AND is_main=true";
    prepared_execute(db, sql, &[&collocations, &dict.0]).await?;
    Ok(())
}

fn collocation_matches(collocation: &Vec<Dict>, kana: &str) -> bool {
    collocation
        .iter()
        .find(|i| !i.kanji && i.reading.contains(kana))
        .map(|_| true)
        .unwrap_or_default()
}

async fn get_kana(db: &Pool, gd: &GenDict) -> Result<String, Error> {
    let res: String = prepared_query_one(
        db,
        "SELECT reading FROM dict WHERE sequence=$1 AND kanji=false LIMIT 1",
        &[&gd.0],
    )
    .await?;
    Ok(res)
}

/// Clear existinig collections
async fn clear(db: &Pool) -> Result<(), Error> {
    let sql = "UPDATE dict SET collocations=NULL";
    prepared_execute(db, sql, &[]).await?;
    Ok(())
}
