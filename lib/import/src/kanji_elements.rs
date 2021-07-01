use deadpool_postgres::Pool;
use itertools::Itertools;
use models::kanji;
use parse::kanji_ele;

/// Import kanji elements
pub async fn import(db: &Pool, path: &str) {
    kanji::clear_kanji_elements(db).await.unwrap();
    println!("Importing kanji elements...");

    for elements in kanji_ele::parse(&path).chunks(200).into_iter() {
        kanji::insert_kanji_parts(db, &elements.into_iter().collect_vec())
            .await
            .expect("db error");
    }
}
