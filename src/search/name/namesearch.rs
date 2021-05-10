use super::SearchMode;
use crate::{error::Error, japanese::JapaneseExt, models::name::Name, search::Search, DbPool};

use tokio_diesel::*;

/// Defines the structure of a
/// name based search
#[derive(Clone)]
pub struct NameSearch<'a> {
    search: Search<'a>,
    db: &'a DbPool,
    limit: i64,
}

impl<'a> NameSearch<'a> {
    pub fn new(db: &'a DbPool, query: &'a str) -> Self {
        Self {
            search: Search::new(query, SearchMode::Exact),
            db,
            limit: 0,
        }
    }

    pub fn with_limit(&mut self, limit: i64) -> &mut Self {
        self.limit = limit;
        self
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
        use crate::sql::ExpressionMethods;
        use diesel::prelude::*;

        let query = self.search.query.clone().to_lowercase();
        let like_pred = &query;

        if self.limit == 0 {
            Ok(if query.is_kanji() {
                // Only need to search in kana
                name.filter(kanji.text_search(&like_pred))
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_kana() {
                // Only need to search in kanji
                name.filter(kana.text_search(&like_pred))
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_japanese() {
                // Search in both, kana & kanji
                name.filter(
                    kanji
                        .text_search(&like_pred)
                        .or(kana.text_search(&like_pred)),
                )
                .get_results_async(&self.db)
                .await?
            } else {
                // Search in transcriptions
                name.filter(transcription.text_search(&like_pred))
                    .get_results_async(&self.db)
                    .await?
            })
        } else {
            Ok(if query.is_kanji() {
                // Only need to search in kana
                name.filter(kanji.text_search(&like_pred))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_kana() {
                // Only need to search in kanji
                name.filter(kana.text_search(&like_pred))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_japanese() {
                // Search in both, kana & kanji
                name.filter(
                    kanji
                        .text_search(&like_pred)
                        .or(kana.text_search(&like_pred)),
                )
                .limit(self.limit)
                .get_results_async(&self.db)
                .await?
            } else {
                // Search in transcriptions
                name.filter(transcription.text_search(&like_pred))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            })
        }
    }
}
