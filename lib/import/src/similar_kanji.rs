use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use deadpool_postgres::Pool;
use models::kanji;

/// Import similar kanji file
pub async fn import(db: &Pool, path: &str) {
    println!("Clearing old similar kanji data");
    kanji::clear_similar_kanji(&db)
        .await
        .expect("failed to clear old similar kanji");

    println!("Importing similar kanji");
    let file = File::open(path).expect("Did not find similar kanji file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut chars = line.chars();

        let character = match chars.next() {
            Some(c) => c,
            None => continue,
        };

        let kanji: Vec<char> = chars.collect();
        if kanji.is_empty() {
            continue;
        }

        kanji::set_similarkanji(db, character, &kanji)
            .await
            .expect("Failed to update similar kanji");
    }
}
