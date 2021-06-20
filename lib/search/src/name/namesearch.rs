use super::{super::Search, SearchMode};
use deadpool_postgres::Pool;
use error::Error;
use japanese::JapaneseExt;
use models::{kanji::reading::KanjiReading, name::Name};

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
        let mut query = String::from("SELECT * FROM name WHERE transcription &@ $1 ");
        if query.len() < 3 {
            query.push_str("ORDER BY LENGTH(transcription) LIMIT 20");
        }

        let client = self.db.get().await?;

        let prepared = client.prepare_cached(&query).await?;
        let rows = client.query(&prepared, &[&self.search.query]).await?;

        let res = rows.into_iter().map(|i| Name::from(i)).collect();

        Ok(res)
    }

    pub async fn kanji_search(&self, kanji: &KanjiReading) -> Result<Vec<Name>, Error> {
        let db = self.db.get().await?;

        let prepared = db
            .prepare_cached("SELECT * FROM name WHERE kanji &@ $1 AND kana &@ $2 LIMIT 10")
            .await?;

        let res = db
            .query(&prepared, &[&kanji.literal.to_string(), &kanji.reading])
            .await?
            .into_iter()
            .map(|i| Name::from(i))
            .collect();

        Ok(res)
    }

    /// Search name by japanese
    pub async fn search_native(&self, query: &str) -> Result<Vec<Name>, Error> {
        let db = self.db.get().await?;

        let (where_, column) = if query.is_kanji() {
            ("kanji &@ $1", "kanji")
        } else if query.is_kana() {
            ("kana &@ $1", "kana")
        } else if query.is_japanese() {
            ("(kana &@ $1 OR kanji &@ $1)", "kana")
        } else {
            ("transcription &@ $1", "transcription")
        };

        let prepared = db
            .prepare_cached(&format!(
                "SELECT * FROM name WHERE {} ORDER BY LENGTH({}) LIMIT $2",
                where_, column
            ))
            .await?;

        let limit = if query.len() < 3 && self.limit == 0 {
            20
        } else if self.limit > 0 {
            self.limit
        } else {
            100
        };

        Ok(db
            .query(&prepared, &[&query, &limit])
            .await?
            .into_iter()
            .map(|i| Name::from(i))
            .collect())
    }
}
