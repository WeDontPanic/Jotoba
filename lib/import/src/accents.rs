use std::io::{stdout, Write};

use deadpool_postgres::Pool;
use itertools::Itertools;
use models::dict;
use parse::accents;

/// Import accent resources
pub async fn import(db: &Pool, path: &str) {
    println!("Importing pitch accents...");

    let (count, iter) = accents::parse(path).expect("Parse error");

    let mut counter = 0;
    for pitch in iter.chunks(400).into_iter() {
        let pitch: Vec<_> = pitch.collect();
        let pitch_len = pitch.len();
        dict::update_accents(&db, pitch.into_iter()).await.unwrap();
        counter += pitch_len;
        print!("\rImporting pitch {}/{}", counter, count);
        stdout().flush().ok();
    }
}
