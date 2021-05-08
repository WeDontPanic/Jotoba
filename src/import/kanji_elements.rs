use crate::DbPool;
use crate::{models::kanji, parse::kanji_ele};

/// Import radicals
pub async fn import(db: &DbPool, path: String) {
    kanji::clear_kanji_elements(db).await.unwrap();
    println!("Importing kanji elements...");

    for element in kanji_ele::parse(&path) {
        kanji::insert_kanji_part(db, element)
            .await
            .expect("db error");
    }
}
