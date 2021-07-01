use deadpool_postgres::Pool;
use itertools::Itertools;
use japanese::JapaneseExt;
use tokio_postgres::Row;

use super::{order::NativeOrder, result};
use error::Error;
use models::sentence::{Sentence, Translation};
use parse::jmdict::languages::Language;

/// The default limit of sentence results. This doesn't represent the max count of sentences being
/// shown to the user but to reduce weight on the DB
const DEFAULT_LIMIT: i64 = 100;

/// The default limit for the items to display
const DISPLAY_LIMIT: i64 = 10;

#[derive(Clone, Copy)]
pub(super) struct SentenceSearch<'a> {
    db: &'a Pool,
    query: &'a str,
    target_lang: Language,
    offset: i32,
    limit: i64,
}

impl<'a> SentenceSearch<'a> {
    pub(super) fn new(db: &'a Pool, query: &'a str, target_lang: Language) -> Self {
        Self {
            db,
            query,
            target_lang,
            limit: 0,
            offset: 0,
        }
    }

    fn get_display_limit(&self) -> i64 {
        if self.limit > 0 {
            self.limit
        } else {
            DISPLAY_LIMIT
        }
    }

    fn get_limit(&self) -> i64 {
        if self.limit > 0 {
            self.limit
        } else {
            DEFAULT_LIMIT
        }
    }

    /// Does the actual search magic and searches for the input
    async fn search_query(&self, query_str: &str) -> Result<Vec<Row>, Error> {
        let db = self.db.get().await?;

        let mut sql_query: String = "SELECT * FROM sentence 
            INNER JOIN sentence_translation ON sentence_translation.sentence_id = sentence.id 
            WHERE (sentence_translation.language = $1 OR sentence_translation.language = $2)"
            .into();

        // Choose the right column to apply the query to
        if query_str.is_kana() {
            sql_query.push_str(" AND kana &@ $3 ");
        } else if query_str.is_japanese() {
            sql_query.push_str(" AND sentence.content &@ $3 ");
        } else {
            sql_query.push_str(" AND sentence_translation.content &@ $3 ");
        }

        sql_query.push_str("LIMIT $4 OFFSET $5");

        let statement = db.prepare_cached(&sql_query).await?;

        Ok(db
            .query(
                &statement,
                &[
                    &self.target_lang,
                    &Language::English,
                    &query_str,
                    &(self.get_limit() as i64),
                    &(self.offset as i64),
                ],
            )
            .await?)
    }

    /// Finds sentences based by japanese input
    pub(super) async fn by_jp(&self) -> Result<Vec<result::Sentence>, Error> {
        let res = self.search_query(&self.query).await?;
        let res: Vec<(Sentence, Translation)> = res.into_iter().map(|i| from_join(i)).collect();

        // Serach for sentences where
        let dict_res = self.get_dict_matches(self.query).await?;

        let mut res: Vec<(Sentence, Translation)> = res.into_iter().chain(dict_res).collect_vec();

        // Remove duplicates which could come from `dict_res`
        res.dedup_by(|a, b| a.0.id == b.0.id);

        let mut res = merge_results(res, self.target_lang);
        NativeOrder::new(&self.query).sort(&mut res);
        res.truncate(self.get_display_limit() as usize);

        Ok(res)
    }

    /// Finds sentences for foreign query input
    pub(super) async fn by_foreign(&self) -> Result<Vec<result::Sentence>, Error> {
        let res = self
            .search_query(self.query)
            .await?
            .into_iter()
            .map(|i| from_join(i))
            .collect();

        let mut res = merge_results(res, self.target_lang);
        res.sort_by(|a, b| a.content.len().cmp(&b.content.len()));
        res.truncate(self.get_display_limit() as usize);

        Ok(res)
    }

    /// Returns a set of sentences if query is found as dictionary word. The sentences getting
    /// returned are those which are mapped to this dictionary word
    async fn get_dict_matches(
        &self,
        query_str: &'a str,
    ) -> Result<Vec<(Sentence, Translation)>, Error> {
        let db = self.db.get().await?;

        // Finds a word in `dicts` which is equal to the `query_str` and returns sentences
        // containing this word
        let sql_query: String = "SELECT sentence.*, sentence_translation.* FROM sentence 
            INNER JOIN sentence_translation ON sentence_translation.sentence_id = sentence.id 
            INNER JOIN sentence_vocabulary ON sentence_vocabulary.sentence_id = sentence.id
            WHERE (sentence_translation.language = $1 OR sentence_translation.language = $2) 
            AND dict_sequence = (SELECT sequence FROM dict WHERE reading = $3 LIMIT 1)
            LIMIT $4 OFFSET $5"
            .into();

        let statement = db.prepare_cached(&sql_query).await?;

        let res = db
            .query(
                &statement,
                &[
                    &self.target_lang,
                    &Language::English,
                    &query_str,
                    &(self.get_limit() as i64),
                    &(self.offset as i64),
                ],
            )
            .await?
            .into_iter()
            .map(|i| from_join(i))
            .collect();

        Ok(res)
    }

    /*
    pub(super) async fn get_vocabularies(
        db: &DbConnection,
        s_id: i32,
    ) -> Result<Vec<(String, SentenceVocabulary)>, Error> {
        use models::schema::dict;
        use models::schema::sentence_vocabulary::dsl::*;
        let e: Vec<(SentenceVocabulary, Dict)> = sentence_vocabulary
            .inner_join(dict::table.on(dict_sequence.eq_all(dict::sequence)))
            .filter(sentence_id.eq_all(s_id))
            .order(id)
            .get_results(db)
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
    */
}

/// Merge Sentences and its translations into one `result::Sentence`
fn merge_results(items: Vec<(Sentence, Translation)>, lang: Language) -> Vec<result::Sentence> {
    items
        .into_iter()
        .group_by(|i| i.0.id)
        .into_iter()
        .filter_map(|(i, right)| {
            let items = right.collect_vec();
            let sent = items.iter().map(|i| i.0.clone()).next().unwrap();
            let trans: Vec<Translation> = items.into_iter().map(|i| i.1).collect();
            let own: Vec<&Translation> = trans.iter().filter(|i| i.language == lang).collect();

            if own.is_empty() {
                return None;
            }

            let english: Vec<&Translation> = trans
                .iter()
                .filter(|i| i.language == Language::English)
                .collect();

            let eng: String = (!english.is_empty())
                .then(|| english[0].content.clone())
                .unwrap_or_else(|| String::from('-'));

            let language = trans
                .iter()
                .filter(|i| i.language != Language::English)
                .map(|i| i.language)
                .next()
                .unwrap_or_default();

            Some(result::Sentence {
                id: i,
                content: sent.content,
                language,
                furigana: sent.furigana,
                translation: own[0].content.clone(),
                eng,
            })
        })
        .collect()
}

/// Takes a result-row of a join between sentence and sentence_translation (in this order) and
/// builds a tuple of Sentence and Translation
fn from_join(row: Row) -> (Sentence, Translation) {
    (
        Sentence {
            id: row.get(0),
            content: row.get(1),
            kana: row.get(2),
            furigana: row.get(3),
        },
        Translation {
            id: row.get(4),
            sentence_id: row.get(5),
            language: row.get(6),
            content: row.get(7),
        },
    )
}
