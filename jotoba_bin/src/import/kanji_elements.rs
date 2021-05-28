use models::{kanji, DbPool};
use parse::kanji_ele;

/// Import kanji elements
pub async fn import(db: &DbPool, path: &str) {
    kanji::clear_kanji_elements(db).await.unwrap();
    println!("Importing kanji elements...");

    for element in kanji_ele::parse(&path) {
        kanji::insert_kanji_part(db, element)
            .await
            .expect("db error");
    }
}
