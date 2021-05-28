use crate::{models::radical, DbPool};
use parse::radicals::search_radicals;

/// Import search radicals
pub async fn import(db: &DbPool, path: &str) {
    println!("Clearing old search-radicals...");
    radical::clear_search_radicals(db).await.unwrap();
    println!("Importing search-radicals...");
    let db = db.get().unwrap();

    for s_radical in search_radicals::parse(path).expect("parsing error") {
        radical::insert_search_radical(&db, s_radical.into()).unwrap();
    }
}
