use diesel::{
    sql_types::{Integer, Text},
    EqAll, JoinOnDsl, QueryDsl,
};
use itertools::Itertools;
use tokio_diesel::AsyncRunQueryDsl;

use super::result;
use crate::{
    error::Error,
    models::{dict::Dict, sentence::SentenceVocabulary},
    parse::jmdict::languages::Language,
    DbPool,
};

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

    // TODO Improve matching for verbs/adjectives
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

    pub(super) async fn get_vocabularies(
        db: &DbPool,
        s_id: i32,
    ) -> Result<Vec<(String, SentenceVocabulary)>, Error> {
        use crate::schema::dict;
        use crate::schema::sentence_vocabulary::dsl::*;
        let e: Vec<(SentenceVocabulary, Dict)> = sentence_vocabulary
            .inner_join(dict::table.on(dict_sequence.eq_all(dict::sequence)))
            .filter(sentence_id.eq_all(s_id))
            .order(id)
            .get_results_async(db)
            .await?;

        let mapped: Vec<(SentenceVocabulary, Dict)> = e
            .into_iter()
            .group_by(|i| i.0.start)
            .into_iter()
            .map(|(_, mut i)| i.next().unwrap())
            .collect_vec();

        Ok(mapped
            .into_iter()
            .map(|(voc, dict)| (dict.reading, voc))
            .collect_vec())
    }
}
