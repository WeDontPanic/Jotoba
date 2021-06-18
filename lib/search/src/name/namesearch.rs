use super::{super::Search, SearchMode};
use error::Error;
use japanese::JapaneseExt;
use models::sql::{length, ExpressionMethods};
use models::{kanji::reading::KanjiReading, name::Name, DbConnection};

use diesel::prelude::*;

/// Defines the structure of a
/// name based search
#[derive(Clone)]
pub struct NameSearch<'a> {
    search: Search<'a>,
    db: &'a DbConnection,
    limit: i64,
}

impl<'a> NameSearch<'a> {
    pub fn new(db: &'a DbConnection, query: &'a str) -> Self {
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
        use diesel::prelude::*;
        use models::schema::name::dsl::*;

        let query = self.search.query;
        let like_pred = self.search.mode.to_like(&query);

        Ok(if query.len() < 3 {
            name.filter(transcription.text_search(&like_pred))
                .order(length(transcription))
                .limit(20)
                .get_results(self.db)?
        } else {
            name.filter(transcription.text_search(&like_pred))
                .get_results(self.db)?
        })
    }

    pub async fn kanji_search(&self, kanji: &KanjiReading) -> Result<Vec<Name>, Error> {
        use models::schema::name;

        Ok(name::table
            .filter(name::kanji.text_search(kanji.literal.to_string()))
            .filter(name::kana.text_search(&kanji.reading))
            .get_results(self.db)?)
    }

    /// Search name by japanese
    pub async fn search_native(&self, query: &str) -> Result<Vec<Name>, Error> {
        use models::schema::name::dsl::*;

        if self.limit == 0 {
            // Search in both, kana & kanji
            if utils::real_string_len(query) < 3 {
                Ok(if query.is_kanji() {
                    // Only need to search in kana
                    name.filter(kanji.text_search(query))
                        .order(models::sql::Nullable::length(kanji))
                        .limit(20)
                        .get_results(self.db)?
                } else if query.is_kana() {
                    // Only need to search in kanji
                    name.filter(kana.text_search(query))
                        .order(length(kana))
                        .limit(20)
                        .get_results(self.db)?
                } else {
                    name.filter(kanji.text_search(query).or(kana.text_search(query)))
                        .order(length(kana))
                        .limit(20)
                        .get_results(self.db)?
                })
            } else {
                Ok(if query.is_kanji() {
                    // Only need to search in kana
                    name.filter(kanji.text_search(query)).get_results(self.db)?
                } else if query.is_kana() {
                    // Only need to search in kanji
                    name.filter(kana.text_search(query)).get_results(self.db)?
                } else if query.is_japanese() {
                    // Search in both, kana & kanji
                    name.filter(kanji.text_search(query).or(kana.text_search(query)))
                        .get_results(self.db)?
                } else {
                    // Search in transcriptions
                    name.filter(transcription.text_search(query))
                        .get_results(self.db)?
                })
            }
        } else {
            Ok(if query.is_kanji() {
                // Only need to search in kana
                name.filter(kanji.text_search(query))
                    .limit(self.limit)
                    .get_results(self.db)?
            } else if query.is_kana() {
                // Only need to search in kanji
                name.filter(kana.text_search(query))
                    .limit(self.limit)
                    .get_results(self.db)?
            } else if query.is_japanese() {
                // Search in both, kana & kanji
                name.filter(kanji.text_search(query).or(kana.text_search(query)))
                    .limit(self.limit)
                    .get_results(self.db)?
            } else {
                // Search in transcriptions
                name.filter(transcription.text_search(query))
                    .limit(self.limit)
                    .get_results(self.db)?
            })
        }
    }
}
