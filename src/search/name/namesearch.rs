use super::SearchMode;
use crate::{
    error::Error,
    japanese::JapaneseExt,
    models::name::Name,
    search::{query::KanjiReading, Search},
    sql::ExpressionMethods,
    DbPool,
};

use diesel::prelude::*;
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
            .filter(transcription.text_search(&like_pred))
            .get_results_async(&self.db)
            .await?)
    }

    pub async fn kanji_search(&self, kanji: &KanjiReading) -> Result<Vec<Name>, Error> {
        use crate::schema::name;

        let res: Vec<Name> = name::table
            .filter(name::kanji.text_search(kanji.literal.to_string()))
            .filter(name::kana.text_search(&kanji.reading))
            .get_results_async(&self.db)
            .await?;

        Ok(res)
    }

    /// Search name by japanese
    pub async fn search_native(&self) -> Result<Vec<Name>, Error> {
        use crate::schema::name::dsl::*;

        let query = &self.search.query;

        if self.limit == 0 {
            Ok(if query.is_kanji() {
                // Only need to search in kana
                name.filter(kanji.text_search(query))
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_kana() {
                // Only need to search in kanji
                name.filter(kana.text_search(query))
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_japanese() {
                // Search in both, kana & kanji
                name.filter(kanji.text_search(query).or(kana.text_search(query)))
                    .get_results_async(&self.db)
                    .await?
            } else {
                // Search in transcriptions
                name.filter(transcription.text_search(query))
                    .get_results_async(&self.db)
                    .await?
            })
        } else {
            Ok(if query.is_kanji() {
                // Only need to search in kana
                name.filter(kanji.text_search(query))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_kana() {
                // Only need to search in kanji
                name.filter(kana.text_search(query))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else if query.is_japanese() {
                // Search in both, kana & kanji
                name.filter(kanji.text_search(query).or(kana.text_search(query)))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            } else {
                // Search in transcriptions
                name.filter(transcription.text_search(query))
                    .limit(self.limit)
                    .get_results_async(&self.db)
                    .await?
            })
        }
    }
}
