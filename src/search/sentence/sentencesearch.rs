use diesel::sql_types::{Integer, Text};
use tokio_diesel::AsyncRunQueryDsl;

use super::result;
use crate::{error::Error, parse::jmdict::languages::Language, DbPool};

/// The default limit of sentence results
const DEFAULT_LIMIT: i32 = 10;

#[derive(Clone, Copy)]
pub(super) struct SentenceSearch<'a> {
    db: &'a DbPool,
    query: &'a str,
    target_lang: Language,
    offset: i32,
    limit: i32,
}

impl<'a> SentenceSearch<'a> {
    pub(super) fn new(db: &'a DbPool, query: &'a str, target_lang: Language) -> Self {
        Self {
            db,
            query,
            target_lang,
            limit: 0,
            offset: 0,
        }
    }

    fn get_limit(&self) -> i32 {
        if self.limit > 0 {
            self.limit
        } else {
            DEFAULT_LIMIT
        }
    }

    pub(super) async fn by_jp(&self) -> Result<Vec<result::Sentence>, Error> {
        let query = include_str!("../../../sql/find_sentence_jp.sql");

        let lang: i32 = self.target_lang.into();

        Ok(diesel::sql_query(query)
            .bind::<Text, _>(&self.query)
            .bind::<Integer, _>(&self.offset)
            .bind::<Integer, _>(&self.get_limit())
            .bind::<Integer, _>(&lang)
            .load_async(self.db)
            .await?)
    }

    pub(super) async fn by_foreign(&self) -> Result<Vec<result::Sentence>, Error> {
        let query = include_str!("../../../sql/find_sentence_foreign.sql");

        let lang: i32 = self.target_lang.into();

        Ok(diesel::sql_query(query)
            .bind::<Text, _>(&self.query)
            .bind::<Integer, _>(&self.offset)
            .bind::<Integer, _>(&self.get_limit())
            .bind::<Integer, _>(&lang)
            .load_async(self.db)
            .await?)
    }
}
