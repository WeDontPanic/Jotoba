use diesel::prelude::*;
use diesel::{
    EqAll, JoinOnDsl, QueryDsl,
};
use itertools::Itertools;
use tokio_diesel::AsyncRunQueryDsl;

use super::result;
use crate::{
    error::Error,
    models::{
        dict::Dict,
        sentence::{Sentence, SentenceVocabulary, Translation},
    },
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
        use crate::schema::sentence::dsl::*;
        use crate::schema::sentence_translation;
        use crate::sql::ExpressionMethods;

        let res: Vec<(Sentence, Translation)> = sentence
            .inner_join(sentence_translation::table)
            .filter(
                sentence_translation::language
                    .eq_all(self.target_lang)
                    .or(sentence_translation::language.eq_all(Language::English)),
            )
            .filter(content.text_search(self.query))
            .offset(self.offset as i64)
            .load_async(self.db)
            .await?;

        let mut res = merge_results(res, self.target_lang);
        res.sort_by(|a, b| a.content.len().cmp(&b.content.len()));
        res.truncate(self.get_limit() as usize);

        Ok(res)
    }

    pub(super) async fn by_foreign(&self) -> Result<Vec<result::Sentence>, Error> {
        use crate::schema::sentence::dsl::*;
        use crate::schema::sentence_translation;
        use crate::sql::ExpressionMethods;

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
                .unwrap_or(String::from("-"));

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
