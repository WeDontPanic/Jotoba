use super::{
    result::word::{Item, Reading},
    Search, SearchMode,
};
use crate::{
    error::Error,
    models::{dict::Dict, sense},
    parse::jmdict::{information::Information, languages::Language, priority::Priority},
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

sql_function! {
    fn lower(a: diesel::types::VarChar) -> diesel::types::VarChar;
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
    pub async fn search_by_glosses(&mut self) -> Result<Vec<Item>, Error> {
        // Load sequence ids to display
        let seq_ids = self.get_sequence_ids_by_glosses().await?;
        self.get_results(&seq_ids).await
    }

    /// Searches by native
    pub async fn search_native(&mut self) -> Result<Vec<Item>, Error> {
        // Load sequence ids to display
        let seq_ids = self.get_sequence_ids_by_native().await?;
        Ok(self
            .get_results(&seq_ids)
            .await?
            .into_iter()
            .filter(|i| self.post_search_check(&i))
            .collect_vec())
    }

    fn post_search_check(&self, item: &Item) -> bool {
        if self.kana_only && item.reading.kanji.is_some() {
            false
        } else {
            true
        }
    }

    /// Get search results of seq_ids
    async fn get_results(&self, seq_ids: &Vec<i32>) -> Result<Vec<Item>, Error> {
        // Request Redings and Senses in parallel
        let (word_items, senses): (Vec<Item>, Vec<sense::Sense>) =
            futures::try_join!(self.load_readings(&seq_ids), self.load_senses(&seq_ids))?;

        Ok(Self::merge_words_with_senses(word_items, senses))
    }

    /// Merge word_items with its senses
    fn merge_words_with_senses(word_items: Vec<Item>, senses: Vec<sense::Sense>) -> Vec<Item> {
        // Map result into a usable word::Item an return it
        word_items
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

        let language_predicate = language
            .eq(Language::English)
            .or(language.eq(Language::German));

        // Wait for tokio-diesel to support boxed queries #20
        if self.search.limit > 0 {
            if self.ignore_case {
                Ok(sense
                    .select(sequence)
                    .filter(
                        match self.search.mode {
                            SearchMode::Exact => lower(gloss).like(query),
                            SearchMode::Variable => lower(gloss).like(format!("%{}%", query)),
                            SearchMode::LeftVariable => lower(gloss).like(format!("%{}", query)),
                            SearchMode::RightVariable => lower(gloss).like(format!("{}%", query)),
                        }
                        .and(language_predicate),
                    )
                    .limit(self.search.limit as i64)
                    .get_results_async(&self.db)
                    .await?)
            } else {
                Ok(sense
                    .select(sequence)
                    .filter(
                        match self.search.mode {
                            SearchMode::Exact => gloss.like(query),
                            SearchMode::Variable => gloss.like(format!("%{}%", query)),
                            SearchMode::LeftVariable => gloss.like(format!("%{}", query)),
                            SearchMode::RightVariable => gloss.like(format!("{}%", query)),
                        }
                        .and(language_predicate),
                    )
                    .limit(self.search.limit as i64)
                    .get_results_async(&self.db)
                    .await?)
            }
        } else {
            if self.ignore_case {
                Ok(sense
                    .select(sequence)
                    .filter(
                        match self.search.mode {
                            SearchMode::Exact => lower(gloss).like(query),
                            SearchMode::Variable => lower(gloss).like(format!("%{}%", query)),
                            SearchMode::LeftVariable => lower(gloss).like(format!("%{}", query)),
                            SearchMode::RightVariable => lower(gloss).like(format!("{}%", query)),
                        }
                        .and(language_predicate),
                    )
                    .get_results_async(&self.db)
                    .await?)
            } else {
                Ok(sense
                    .select(sequence)
                    .filter(
                        match self.search.mode {
                            SearchMode::Exact => gloss.like(query),
                            SearchMode::Variable => gloss.like(format!("%{}%", query)),
                            SearchMode::LeftVariable => gloss.like(format!("%{}", query)),
                            SearchMode::RightVariable => gloss.like(format!("{}%", query)),
                        }
                        .and(language_predicate),
                    )
                    .get_results_async(&self.db)
                    .await?)
            }
        }
    }

    /// Find the sequence ids of the results to load
    async fn get_sequence_ids_by_native(&mut self) -> Result<Vec<i32>, Error> {
        use crate::schema::dict::dsl::*;

        let predicate = {
            match self.search.mode {
                SearchMode::Exact => reading.like(self.search.query.to_owned()),
                SearchMode::Variable => reading.like(format!("%{}%", self.search.query)),
                SearchMode::LeftVariable => reading.like(format!("%{}", self.search.query)),
                SearchMode::RightVariable => reading.like(format!("{}%", self.search.query)),
            }
        };

        // Wait for tokio-diesel to support boxed queries #20
        if self.search.limit > 0 {
            Ok(dict
                .select(sequence)
                .filter(predicate)
                .limit(self.search.limit as i64)
                .get_results_async(&self.db)
                .await?)
        } else {
            Ok(dict
                .select(sequence)
                .filter(predicate)
                .get_results_async(&self.db)
                .await?)
        }
    }

    /// Load all senses for the sequence ids
    async fn load_senses(&self, sequence_ids: &Vec<i32>) -> Result<Vec<sense::Sense>, Error> {
        use crate::schema::sense as sense_schema;

        // Always search by a language.
        let lang = self.language.unwrap_or(Language::default());

        let senses: Vec<sense::Sense> = sense_schema::table
            .filter(
                sense_schema::sequence.eq_any(sequence_ids).and(
                    sense_schema::language
                        .eq(lang)
                        .or(sense_schema::language.eq(Language::default())),
                ),
            )
            .get_results_async(&self.db)
            .await?;

        Ok(senses)
    }

    /// Load readings for all sequences
    async fn load_readings(&self, sequence_ids: &Vec<i32>) -> Result<Vec<Item>, Error> {
        use crate::schema::dict as dict_schema;

        // load dicts from DB
        let dicts: Vec<Dict> = dict_schema::table
            .filter(dict_schema::sequence.eq_any(sequence_ids))
            .order_by(dict_schema::id)
            .get_results_async(&self.db)
            .await?;

        Ok(dicts
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

                Item {
                    reading,
                    priorities,
                    information,
                    sequence: seq,
                    ..Default::default()
                }
            })
            .collect_vec())
    }
}
