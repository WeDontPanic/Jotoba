use super::result::{Reading, Word};

use crate::{
    error::Error,
    models::{dict::Dict, sense},
    parse::jmdict::{information::Information, languages::Language, priority::Priority},
    search::{Search, SearchMode},
    DbPool,
};

use diesel::prelude::*;
use itertools::Itertools;
use tokio_diesel::*;

/// Defines the structure of a
/// word based search
#[derive(Clone)]
pub struct WordSearch<'a> {
    search: Search<'a>,
    db: &'a DbPool,
    language: Option<Language>,
    ignore_case: bool,
    kana_only: bool,
}

impl<'a> WordSearch<'a> {
    pub fn new(db: &'a DbPool, query: &'a str) -> Self {
        Self {
            search: Search::new(query, SearchMode::Variable),
            db,
            language: None,
            ignore_case: false,
            kana_only: false,
        }
    }

    /// Use a specific language for the search
    pub fn with_case_insensitivity(&mut self, ignore_case: bool) -> &mut Self {
        self.ignore_case = ignore_case;
        self
    }

    /// Ignore the case of the input
    pub fn with_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    /// Use a specific mode for the search
    pub fn with_mode(&mut self, mode: SearchMode) -> &mut Self {
        self.search.mode = mode;
        self
    }

    /// Use a specific limit for the search
    pub fn with_limit(&mut self, limit: u16) -> &mut Self {
        self.search.limit = limit;
        self
    }

    /// Use a specific limit for the search
    pub fn with_kana_only(&mut self, kana_only: bool) -> &mut Self {
        self.kana_only = kana_only;
        self
    }

    /// Searches by translations
    pub async fn search_by_glosses(&mut self) -> Result<Vec<Word>, Error> {
        // Load sequence ids to display
        let seq_ids = self.get_sequence_ids_by_glosses().await?;

        // always search by a language.
        let lang = self.language.unwrap_or(Language::default());

        Self::load_words_by_seq(&self.db, &seq_ids, lang).await
    }

    /// Searches by native
    pub async fn search_native(&mut self) -> Result<Vec<Word>, Error> {
        // Load sequence ids to display
        let seq_ids = self.get_sequence_ids_by_native().await?;

        // always search by a language.
        let lang = self.language.unwrap_or(Language::default());

        Ok(Self::load_words_by_seq(&self.db, &seq_ids, lang)
            .await?
            .into_iter()
            .filter(|i| self.post_search_check(&i))
            .collect_vec())
    }

    fn post_search_check(&self, item: &Word) -> bool {
        if self.kana_only && item.reading.kanji.is_some() {
            false
        } else {
            true
        }
    }

    /// Get search results of seq_ids
    pub async fn load_words_by_seq(
        db: &DbPool,
        seq_ids: &Vec<i32>,
        lang: Language,
    ) -> Result<Vec<Word>, Error> {
        // Request Redings and Senses in parallel
        let (dicts, senses): (Vec<Dict>, Vec<sense::Sense>) = futures::try_join!(
            Self::load_dictionaries(&db, &seq_ids),
            Self::load_senses(&db, &seq_ids, lang)
        )?;

        let word_items = convert_dicts_to_words(dicts);

        //Self::load_readings(&db, &seq_ids),
        Ok(merge_words_with_senses(word_items, senses))
    }

    /// Find the sequence ids of the results to load
    async fn get_sequence_ids_by_glosses(&mut self) -> Result<Vec<i32>, Error> {
        use crate::schema::sense::dsl::*;

        let query = {
            if self.ignore_case {
                self.search.query.to_lowercase()
            } else {
                self.search.query.to_string()
            }
        };

        use diesel::dsl::sql;

        // Since boxed queries don't work with tokio-diesel
        // this has to be done. If #20 gets resolved, change this !!
        let mut filter = String::new();

        // Main condidion
        let like = self.search.mode.to_like(query);
        if self.ignore_case {
            filter.push_str(format!("lower(gloss) like '{}'", like).as_str());
        } else {
            filter.push_str(format!("gloss like '{}'", like).as_str());
        }

        // Language filter
        let lang: i32 = self.language.unwrap_or_default().into();
        if self.language.unwrap_or_default() == Language::English {
            filter.push_str(format!(" AND language = {}", lang).as_str());
        } else {
            filter.push_str(format!(" AND (language = {} or language = 1)", lang).as_str());
        }

        // Limit
        if self.search.limit > 0 {
            filter.push_str(format!(" limit {}", self.search.limit).as_str());
        }

        return Ok(sense
            .select(sequence)
            .filter(sql(&filter))
            .get_results_async(&self.db)
            .await?);
    }

    /// Find the sequence ids of the results to load
    async fn get_sequence_ids_by_native(&mut self) -> Result<Vec<i32>, Error> {
        use crate::schema::dict::dsl::*;

        let query = dict
            .select(sequence)
            .filter(reading.like(self.search.mode.to_like(self.search.query.to_string())));

        // Wait for tokio-diesel to support boxed queries #20
        if self.search.limit > 0 {
            Ok(query
                .limit(self.search.limit as i64)
                .get_results_async(&self.db)
                .await?)
        } else {
            Ok(query.get_results_async(&self.db).await?)
        }
    }

    /// Load all senses for the sequence ids
    async fn load_senses(
        db: &DbPool,
        sequence_ids: &Vec<i32>,
        lang: Language,
    ) -> Result<Vec<sense::Sense>, Error> {
        use crate::schema::sense as sense_schema;

        Ok(sense_schema::table
            .filter(sense_schema::sequence.eq_any(sequence_ids))
            .filter(
                sense_schema::language
                    .eq(lang)
                    .or(sense_schema::language.eq(Language::default())),
            )
            .get_results_async(db)
            .await?)
    }

    /// Load Dictionaries of all sequences
    async fn load_dictionaries(db: &DbPool, sequence_ids: &Vec<i32>) -> Result<Vec<Dict>, Error> {
        use crate::schema::dict as dict_schema;

        Ok(dict_schema::table
            .filter(dict_schema::sequence.eq_any(sequence_ids))
            .order_by(dict_schema::id)
            .get_results_async(&db)
            .await?)
    }
}

/// Convert dictionaries to Words
///
/// Since glosses aren't provided, they have
/// to be added later on using `merge_words_with_senses`
pub fn convert_dicts_to_words(dicts: Vec<Dict>) -> Vec<Word> {
    dicts
        .into_iter()
        .group_by(|i| i.sequence)
        .into_iter()
        .map(|(seq, dicts)| {
            let mut reading = Reading {
                sequence: seq,
                ..Default::default()
            };
            let mut priorities: Option<Vec<Priority>> = None;
            let mut information: Option<Vec<Information>> = None;

            dicts.for_each(|dict| {
                if priorities.is_none() && dict.priorities.is_some() {
                    priorities = dict.priorities.clone();
                }
                if information.is_none() && dict.information.is_some() {
                    information = dict.information.clone();
                }

                if reading.kana.is_none() && !dict.kanji {
                    reading.kana = Some(dict);
                    return;
                }

                if reading.kanji.is_none() && dict.kanji {
                    reading.kanji = Some(dict);
                    return;
                }

                reading.alternative.push(dict);
            });

            Word {
                reading,
                priorities,
                information,
                sequence: seq,
                ..Default::default()
            }
        })
        .collect_vec()
}

/// Merge word_items with its senses
pub fn merge_words_with_senses(words: Vec<Word>, senses: Vec<sense::Sense>) -> Vec<Word> {
    // Map result into a usable word::Word an return it
    words
        .into_iter()
        .map(|mut word| {
            word.senses = senses
                .iter()
                .filter(|i| i.sequence == word.sequence)
                .cloned()
                .into_iter()
                // Create a Vec<Sense> grouped by the gloss position
                .group_by(|i| i.gloss_pos)
                .into_iter()
                .map(|(_, j)| j.collect_vec().into())
                .collect_vec();

            word
        })
        .collect_vec()
}
