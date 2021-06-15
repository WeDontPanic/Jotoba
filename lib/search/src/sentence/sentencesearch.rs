use diesel::{prelude::*, EqAll, QueryDsl};
use itertools::Itertools;
use japanese::JapaneseExt;
use tokio_diesel::AsyncRunQueryDsl;

use super::result;
use error::Error;
use models::{
    sentence::{Sentence, SentenceVocabulary, Translation},
    DbPool,
};
use parse::jmdict::languages::Language;

/// The default limit of sentence results. This doesn't represent the max count of sentences being
/// shown to the user but to reduce weight on the DB
const DEFAULT_LIMIT: i64 = 100;

#[derive(Clone, Copy)]
pub(super) struct SentenceSearch<'a> {
    db: &'a DbPool,
    query: &'a str,
    target_lang: Language,
    offset: i32,
    limit: i64,
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

    fn get_limit(&self) -> i64 {
        if self.limit > 0 {
            self.limit
        } else {
            DEFAULT_LIMIT
        }
    }

    /// Finds sentences based by japanese input
    pub(super) async fn by_jp(&self) -> Result<Vec<result::Sentence>, Error> {
        use models::schema::sentence::dsl::*;
        use models::schema::sentence_translation;
        use models::sql::ExpressionMethods;

        let res = if self.query.is_kana() {
            sentence
                .inner_join(sentence_translation::table)
                .filter(
                    sentence_translation::language
                        .eq_all(self.target_lang)
                        .or(sentence_translation::language.eq_all(Language::English)),
                )
                .filter(kana.text_search(self.query))
                .limit(self.get_limit())
                .offset(self.offset as i64)
                .load_async::<(Sentence, Translation)>(self.db)
                .await?
        } else {
            sentence
                .inner_join(sentence_translation::table)
                .filter(
                    sentence_translation::language
                        .eq_all(self.target_lang)
                        .or(sentence_translation::language.eq_all(Language::English)),
                )
                .filter(content.text_search(self.query))
                .offset(self.offset as i64)
                .limit(self.get_limit())
                .load_async::<(Sentence, Translation)>(self.db)
                .await?
        };

        // Serach for sentences where
        let dict_res = self.get_dict_matches(self.query).await?;

        let mut res: Vec<(Sentence, Translation)> = res.into_iter().chain(dict_res).collect_vec();

        // Remove duplicates which could come from `dict_res`
        res.dedup_by(|a, b| a.0.id == b.0.id);

        let mut res = merge_results(res, self.target_lang);
        res.sort_by(|a, b| a.content.len().cmp(&b.content.len()));
        res.truncate(self.get_limit() as usize);

        Ok(res)
    }

    /// Returns a set of sentences if query is found as dictionary word. The sentences getting
    /// returned are those which are mapped to this dictionary word
    async fn get_dict_matches(
        &self,
        query: &'a str,
    ) -> Result<Vec<(Sentence, Translation)>, Error> {
        use models::schema::sentence::dsl::*;
        use models::schema::sentence_translation;
        use models::schema::sentence_vocabulary;

        let seq_id = error::db_to_option(models::dict::get_word_sequence(self.db, query).await)?;
        if seq_id.is_none() {
            return Ok(vec![]);
        }
        let seq_id = seq_id.unwrap();

        let res = sentence
            .inner_join(sentence_translation::table)
            .inner_join(sentence_vocabulary::table)
            .filter(
                sentence_translation::language
                    .eq_all(self.target_lang)
                    .or(sentence_translation::language.eq_all(Language::English)),
            )
            .filter(sentence_vocabulary::dict_sequence.eq_all(seq_id))
            .offset(self.offset as i64)
            .limit(self.get_limit())
            .load_async::<(Sentence, Translation, SentenceVocabulary)>(self.db)
            .await?;

        Ok(res.into_iter().map(|(s, t, _)| (s, t)).collect())
    }

    /// Finds sentences for foreign query input
    pub(super) async fn by_foreign(&self) -> Result<Vec<result::Sentence>, Error> {
        use models::schema::sentence::dsl::*;
        use models::schema::sentence_translation;
        use models::sql::ExpressionMethods;

        let res: Vec<(Sentence, Translation)> = sentence
            .inner_join(sentence_translation::table)
            .filter(
                sentence_translation::language
                    .eq_all(self.target_lang)
                    .or(sentence_translation::language.eq_all(Language::English)),
            )
            .filter(sentence_translation::content.text_search(self.query))
            .offset(self.offset as i64)
            .load_async(self.db)
            .await?;

        let mut res = merge_results(res, self.target_lang);
        res.sort_by(|a, b| a.content.len().cmp(&b.content.len()));
        res.truncate(self.get_limit() as usize);
        Ok(res)
    }

    /*
    pub(super) async fn get_vocabularies(
        db: &DbPool,
        s_id: i32,
    ) -> Result<Vec<(String, SentenceVocabulary)>, Error> {
        use models::schema::dict;
        use models::schema::sentence_vocabulary::dsl::*;
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

            Some(result::Sentence {
                id: i,
                content: sent.content,
                language: Language::German,
                furigana: sent.furigana,
                translation: own[0].content.clone(),
                eng,
            })
        })
        .collect()
}
