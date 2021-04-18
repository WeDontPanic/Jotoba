use crate::{
    error::{self, Error},
    models::kanji::{self, Kanji},
    DbPool,
};

/// Find a kanji by its literal
pub async fn by_literal(db: &DbPool, literal: &str) -> Result<Kanji, Error> {
    kanji::find_by_literal(&db, literal).await.map_err(|i| {
        // Map Diesel not-found errors to Error::NotFound
        if let Error::DbError(db) = i {
            error::map_notfound(db)
        } else {
            i
        }
    })
}
