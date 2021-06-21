use super::{super::Search, SearchMode};
use deadpool_postgres::Pool;
use error::Error;
use japanese::JapaneseExt;
use models::{
    kanji::reading::KanjiReading,
    name::Name,
    queryable::{Queryable, SQL},
};
use utils::real_string_len;

/// Defines the structure of a
/// name based search
#[derive(Clone)]
pub struct NameSearch<'a> {
    search: Search<'a>,
    db: &'a Pool,
    limit: i64,
}

impl<'a> NameSearch<'a> {
    pub fn new(db: &'a Pool, query: &'a str) -> Self {
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
        let query = if self.search.query.len() < 3 {
            Name::select_where_order_limit("transcription &@ $1", "LENGTH(transcription)", 20)
        } else {
            Name::select_where("transcription &@ $1")
        };

        Ok(Name::query(self.db, query, &[&self.search.query], 0).await?)
    }

    pub async fn kanji_search(&self, kanji: &KanjiReading) -> Result<Vec<Name>, Error> {
        Ok(Name::query(
            self.db,
            Name::select_where_limit("kanji &@ $1 AND kana &@ $2", 10),
            &[&kanji.literal.to_string(), &kanji.reading],
            0,
        )
        .await?)
    }

    /// Search name by japanese
    pub async fn search_native(&self, query: &str) -> Result<Vec<Name>, Error> {
        let (where_, column) = if query.is_kanji() {
            ("kanji &@ $1", "kanji")
        } else if query.is_kana() {
            ("kana &@ $1", "kana")
        } else if query.is_japanese() {
            ("(kana &@ $1 OR kanji &@ $1)", "kana")
        } else {
            ("transcription &@ $1", "transcription")
        };

        let limit = if real_string_len(query) < 3 && self.limit == 0 {
            20
        } else if self.limit > 0 {
            self.limit
        } else {
            100
        };

        let sql_query =
            Name::select_where_order_limit(where_, &format!("LENGTH({})", column), limit);
        Ok(Name::query(self.db, sql_query, &[&query], 0).await?)
    }
}
