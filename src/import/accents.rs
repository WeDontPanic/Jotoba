use std::io::Write;

use crate::{models::dict, parse::accents, DbPool};

/// Import jlpt patche file
pub async fn import(db: &DbPool, path: String) {
    println!("Importing pitch accents...");
    let db = db.get().unwrap();

    accents::parse(path, |(kanji, _, pitch), pos, len| {
        dict::update_accents(&db, &kanji, &pitch).unwrap();

        print!("\rImporting pitch {}/{}", pos, len);
        std::io::stdout().flush().ok();
    })
    .unwrap();
}
