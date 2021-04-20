use super::Search;
use crate::{error::Error, japanese::JapaneseExt, models::name::Name, DbPool};
use tokio_diesel::*;

/// Defines the structure of a
/// name based search
#[derive(Clone)]
pub struct NameSearch<'a> {
    search: Search<'a>,
    db: &'a DbPool,
}

impl<'a> NameSearch<'a> {
    pub fn new(db: &'a DbPool, search: Search<'a>) -> Self {
        Self { search, db }
    }

    /// Search name by transcription
    pub async fn search_transcription(&self) -> Result<Vec<Name>, Error> {
        use crate::schema::name::dsl::*;
        use diesel::prelude::*;

        let query = self.search.query;
        let like_pred = self.search.mode.to_like(query);

        Ok(name
            .filter(transcription.like(&like_pred))
            .get_results_async(&self.db)
            .await?)
    }

    /// Search name by japanese
    pub async fn search_native(&self) -> Result<Vec<Name>, Error> {
        use crate::schema::name::dsl::*;
        use diesel::prelude::*;

        let query = self.search.query;
        let like_pred = self.search.mode.to_like(query);

        Ok(if query.is_kanji() {
            // Only need to search in kana
            name.filter(kanji.like(&like_pred))
                .get_results_async(&self.db)
                .await?
        } else if query.is_kana() {
            // Only need to search in kanji
            name.filter(kana.like(&like_pred))
                .get_results_async(&self.db)
                .await?
        } else {
            // Search in both, kana & kanji
            name.filter(kanji.like(&like_pred).or(kana.like(&like_pred)))
                .get_results_async(&self.db)
                .await?
        })
    }
}
