use crate::parse::radicals::search_radicals;
use crate::{models::radical, DbPool};

/// Import search radicals
pub async fn import(db: &DbPool, path: String) {
    println!("Clearing old search-radicals...");
    radical::clear_search_radicals(db).await.unwrap();
    println!("Importing search-radicals...");
    let db = db.get().unwrap();

    for s_radical in search_radicals::parse(&path) {
        radical::insert_search_radical(&db, s_radical.into()).unwrap();
    }
}
