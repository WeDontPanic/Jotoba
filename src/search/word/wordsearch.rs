use super::result::{Reading, Sense, Word};

use crate::{
    error::Error,
    models::{dict::Dict, sense},
    parse::jmdict::{
        information::Information,
        languages::Language,
        part_of_speech::{PartOfSpeech, PosSimple},
        priority::Priority,
    },
    search::{Search, SearchMode},
    sql::ExpressionMethods,
    utils::to_option,
    DbPool,
};
use diesel::sql_types::{Integer, Text};

use diesel::prelude::*;
use futures::future::try_join_all;
use itertools::Itertools;
use tokio_diesel::*;

const MAX_WORDS_TO_HANDLE: usize = 1000;

/// Defines the structure of a
/// word based search
#[derive(Clone)]
pub struct WordSearch<'a> {
    search: Search<'a>,
    db: &'a DbPool,
    language: Option<Language>,
    ignore_case: bool,
    kana_only: bool,
    english_glosses: bool,
    p_o_s_filter: Option<Vec<PosSimple>>,
}

impl<'a> WordSearch<'a> {
    pub fn new(db: &'a DbPool, query: &'a str) -> Self {
        Self {
            search: Search::new(query, SearchMode::Variable),
            db,
            language: None,
            ignore_case: false,
            kana_only: false,
            english_glosses: true,
            p_o_s_filter: None,
        }
    }

    /// Set the query of the search
    pub fn with_query(&mut self, query: &'a str) -> &mut Self {
        self.search.query = query;
        self
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

    /// Only search for kana words
    pub fn with_kana_only(&mut self, kana_only: bool) -> &mut Self {
        self.kana_only = kana_only;
        self
    }

    /// Use a specific limit for the search
    pub fn with_english_glosses(&mut self, english_glosses: bool) -> &mut Self {
        self.english_glosses = english_glosses;
        self
    }

    /// Use part of speech filtetr
    pub fn with_pos_filter(&mut self, filter: &[PosSimple]) -> &mut Self {
        if !filter.is_empty() {
            self.p_o_s_filter = Some(filter.iter().copied().collect_vec());
        }
        self
    }

    /// Searches by translations
    pub async fn search_by_glosses(&mut self) -> Result<Vec<Word>, Error> {
        // Load sequence ids to display
        let seq_ids = self.get_sequence_ids_by_glosses().await?;
        if seq_ids.is_empty() {
            return Ok(vec![]);
        }

        // always search by a language.
        let lang = self.language.unwrap_or_default();

        Self::load_words_by_seq(
            &self.db,
            &seq_ids,
            lang,
            self.english_glosses,
            &self.p_o_s_filter,
        )
        .await
    }

    /// Searches by native
    pub async fn search_native(&mut self) -> Result<Vec<Word>, Error> {
        // Load sequence ids to display
        let seq_ids = self.get_sequence_ids_by_native().await?;

        // always search by a language.
        let lang = self.language.unwrap_or_default();

        Ok(Self::load_words_by_seq(
            &self.db,
            &seq_ids,
            lang,
            self.english_glosses,
            &self.p_o_s_filter,
        )
        .await?
        .into_iter()
        .filter(|i| self.post_search_check(&i))
        .collect_vec())
    }

    fn post_search_check(&self, item: &Word) -> bool {
        if self.kana_only && item.reading.kanji.is_some() {
            return false;
        }

        true
    }

    /// Get search results of seq_ids
    pub async fn load_words_by_seq(
        db: &DbPool,
        seq_ids: &[i32],
        lang: Language,
        include_english: bool,
        pos_filter: &Option<Vec<PosSimple>>,
    ) -> Result<Vec<Word>, Error> {
        if seq_ids.is_empty() {
            return Ok(vec![]);
        }

        // Request Redings and Senses in parallel
        let (dicts, senses): (Vec<Dict>, Vec<sense::Sense>) = futures::try_join!(
            Self::load_dictionaries(&db, &seq_ids),
            Self::load_senses(&db, &seq_ids, lang)
        )?;

        let word_items = convert_dicts_to_words(dicts);

        //Self::load_readings(&db, &seq_ids),
        Ok(merge_words_with_senses(
            word_items,
            senses,
            include_english || lang == Language::default(),
            pos_filter,
        ))
    }

    /// Find the sequence ids of the results to load
    async fn get_sequence_ids_by_glosses(&mut self) -> Result<Vec<i32>, Error> {
        // Since boxed queries don't work with tokio-diesel
        // this has to be done. If #20 gets resolved, change this !!
        let mut filter = String::from("SELECT sequence, length(gloss) as len from sense WHERE");

        // TODO make operator adjustable
        filter.push_str(" gloss &@~ $1 ");

        // Language filter
        let lang: i32 = self.language.unwrap_or_default().into();
        if self.is_default_language() || !self.english_glosses {
            filter.push_str(format!(" AND language = {}", lang).as_str());
        } else {
            filter.push_str(format!(" AND (language = {} or language = 0)", lang).as_str());
        }

        // Add a limit
        if self.search.limit > 0 {
            filter.push_str(format!(" limit {}", self.search.limit).as_str());
        }

        let res: Vec<SearchItemsSql> = diesel::sql_query(&filter)
            .bind::<Text, _>(&self.search.query)
            .get_results_async(&self.db)
            .await?;

        Ok(SearchItemsSql::order(res))
    }

    /// Find the sequence ids of the results to load
    async fn get_sequence_ids_by_native(&mut self) -> Result<Vec<i32>, Error> {
        use crate::schema::dict::dsl::*;
        use crate::sql::length;

        let res: Vec<(i32, i32)> = if self.search.limit > 0 {
            dict.select((sequence, length(reading)))
                .filter(reading.text_search(self.search.query))
                .limit(self.search.limit as i64)
                .get_results_async(&self.db)
                .await?
        } else {
            dict.select((sequence, length(reading)))
                .filter(reading.text_search(self.search.query))
                .get_results_async(&self.db)
                .await?
        };

        let res = res
            .into_iter()
            .map(|i| SearchItemsSql {
                sequence: i.0,
                len: i.1,
            })
            .collect_vec();

        Ok(SearchItemsSql::order(res))
    }

    /// Load all senses for the sequence ids
    async fn load_senses(
        db: &DbPool,
        sequence_ids: &[i32],
        lang: Language,
    ) -> Result<Vec<sense::Sense>, Error> {
        if sequence_ids.is_empty() {
            return Ok(vec![]);
        }

        use crate::schema::sense as sense_schema;
        use diesel::dsl::sql;
        use diesel::ExpressionMethods;

        let lang_i: i32 = lang.into();
        let language = {
            if lang == Language::default() {
                format!(" language = {}", lang_i)
            } else {
                format!(" (language = {} or language = 0)", lang_i)
            }
        };

        Ok(sense_schema::table
            .filter(sense_schema::sequence.eq_any(sequence_ids))
            .filter(sql(&language))
            .order(sense_schema::id)
            .get_results_async(db)
            .await?)
    }

    /// Loads the collocations for all words
    pub async fn load_collocations(
        db: &DbPool,
        words: &mut Vec<Word>,
        language: Language,
    ) -> Result<(), Error> {
        let collocations = try_join_all(
            words
                .iter()
                .map(|i| i.get_reading().load_collocation(db, language)),
        )
        .await?;

        for collocation in collocations {
            let seq = collocation.0;
            let collocations = collocation.1;

            words
                .iter_mut()
                .find(|i| i.sequence == seq)
                .unwrap()
                .collocations = to_option(collocations);
        }

        Ok(())
    }

    /// Load Dictionaries of a single sequence id
    pub async fn load_dictionary(db: &DbPool, sequence_id: i32) -> Result<Vec<Dict>, Error> {
        use crate::schema::dict as dict_schema;

        Ok(dict_schema::table
            .filter(dict_schema::sequence.eq_all(sequence_id))
            .order_by(dict_schema::id)
            .get_results_async(&db)
            .await?)
    }

    /// Load Dictionaries of all sequences
    pub async fn load_dictionaries(db: &DbPool, sequence_ids: &[i32]) -> Result<Vec<Dict>, Error> {
        use crate::schema::dict as dict_schema;
        use diesel::ExpressionMethods;

        if sequence_ids.is_empty() {
            return Ok(vec![]);
        }

        Ok(dict_schema::table
            .filter(dict_schema::sequence.eq_any(sequence_ids))
            .order_by(dict_schema::id)
            .get_results_async(&db)
            .await?)
    }

    /// Returns true if the search will run against
    /// the default language
    fn is_default_language(&self) -> bool {
        self.language
            .map(|i| i == Language::default())
            // No language selected => default
            .unwrap_or(true)
    }
}

#[derive(QueryableByName)]
struct SearchItemsSql {
    #[sql_type = "Integer"]
    sequence: i32,
    #[sql_type = "Integer"]
    len: i32,
}

impl SearchItemsSql {
    fn order(res: Vec<Self>) -> Vec<i32> {
        let mut res = res;
        if res.len() > MAX_WORDS_TO_HANDLE {
            // We can sort by length here because only very short, usually single char searches
            // will result in large amount of sequence ids and the limit is only required to not
            // load too many words at once. The items will be ordered properly later on.
            res.sort_by(|a, b| a.len.cmp(&b.len));
            res.truncate(MAX_WORDS_TO_HANDLE);
        }

        res.into_iter().map(|i| i.sequence).collect_vec()
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
pub fn merge_words_with_senses(
    words: Vec<Word>,
    senses: Vec<sense::Sense>,
    include_english: bool,
    pos_filter: &Option<Vec<PosSimple>>,
) -> Vec<Word> {
    // Map result into a usable word::Word an return it
    words
        .into_iter()
        .filter_map(|mut word| {
            let (english, mut other): (Vec<Sense>, Vec<Sense>) = senses
                .iter()
                .filter(|i| i.sequence == word.sequence)
                .cloned()
                .into_iter()
                // Create a Vec<Sense> grouped by the gloss position
                .group_by(|i| i.gloss_pos)
                .into_iter()
                .map(|(_, j)| {
                    let e: Sense = j.collect_vec().into();
                    e
                })
                .partition(|i| i.language == Language::English);

            // Set other's p_o_s items to the ones from english which are available in each sense
            let unionized_pos = pos_unionized(&english);
            if !unionized_pos.is_empty() {
                other.iter_mut().for_each(|item| {
                    for gloss in item.glosses.iter_mut() {
                        gloss.part_of_speech = unionized_pos.clone();
                    }
                });
            }

            // Set other's misc items to the ones from english if they are all the same
            if all_misc_same(&english) && !english.is_empty() && english[0].misc.is_some() {
                let misc = english[0].misc;
                other.iter_mut().for_each(|item| {
                    item.misc = misc;
                });
            }

            // filter words by part of speach if a filter was provided
            if let Some(pos_filter) = pos_filter {
                if !has_pos(&other, pos_filter) && !has_pos(&english, pos_filter) {
                    return None;
                }
            }

            word.senses = if include_english || other.is_empty() {
                english.into_iter().chain(other).collect_vec()
            } else {
                other
            };

            Some(word)
        })
        .collect_vec()
}

/// Returns true if a vec of senses has at least one Pos provided by the filter
fn has_pos(senses: &[Sense], pos_filter: &[PosSimple]) -> bool {
    for sense in senses.iter() {
        for p in sense.get_pos_simple() {
            if pos_filter.contains(&p) {
                return true;
            }
        }
    }

    false
}

/// Return true if all 'misc' items are of the same value
fn all_misc_same(senses: &[Sense]) -> bool {
    if senses.is_empty() || senses[0].misc.is_none() {
        return false;
    }

    let mut sense_iter = senses.iter();
    let first_misc = sense_iter.next().unwrap().misc;

    for i in sense_iter {
        if i.misc != first_misc {
            return false;
        }
    }

    true
}

/// Return true if all 'part_of_speech' items are of the same value
fn pos_unionized(senses: &[Sense]) -> Vec<PartOfSpeech> {
    if senses.is_empty()
        || senses[0].glosses.is_empty()
        || senses[0].glosses[0].part_of_speech.is_empty()
    {
        return vec![];
    }

    let mut sense_iter = senses.iter();
    let mut pos = sense_iter.next().unwrap().glosses[0].part_of_speech.clone();

    for i in sense_iter {
        pos = crate::utils::union_elements(&pos, &i.glosses[0].part_of_speech)
            .into_iter()
            .cloned()
            .collect_vec();
    }

    pos
}

#[derive(Debug, PartialEq, Clone, QueryableByName)]
pub struct SenqenceSelect {
    #[sql_type = "Integer"]
    pub sequence: i32,
}
