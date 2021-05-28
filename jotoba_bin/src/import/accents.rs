use std::io::Write;

use crate::{models::dict, DbPool};
use parse::accents;

/// Import jlpt patche file
pub async fn import(db: &DbPool, path: &str) {
    println!("Importing pitch accents...");
    let db = db.get().unwrap();

    let (count, iter) = accents::parse(path).expect("Parse error");

    for (pos, pitch) in iter.enumerate() {
        dict::update_accents(&db, pitch).unwrap();
        print!("\rImporting pitch {}/{}", pos, count);
        std::io::stdout().flush().ok();
    }
}
