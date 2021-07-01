use deadpool_postgres::Pool;
use itertools::Itertools;
use models::radical as DbRadical;
use parse::radicals;

/// Import radicals
pub async fn import(db: &Pool, path: &str) {
    println!("Clearing old radicals...");
    DbRadical::clear(db).await.expect("Clearing radials failed");
    println!("Importing radicals...");

    for radical_chunk in radicals::parse(&path).chunks(50).into_iter() {
        DbRadical::insert(db, radical_chunk)
            .await
            .expect("Inserting radical failed");
    }
}
