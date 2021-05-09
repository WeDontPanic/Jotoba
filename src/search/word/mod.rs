#[cfg(feature = "tokenizer")]
pub mod jp_parsing;

mod kanji;
mod order;
pub mod result;
mod wordsearch;

use result::{Item, Word};
pub use wordsearch::WordSearch;

use async_std::sync::Mutex;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::time::SystemTime;

use crate::{
    cache::SharedCache,
    error::Error,
    japanese::JapaneseExt,
    models::kanji::Kanji as DbKanji,
    search::{
        query::{Query, QueryLang},
        search_order::SearchOrder,
        SearchMode,
    },
    utils::real_string_len,
    DbPool,
};

#[cfg(feature = "tokenizer")]
use self::jp_parsing::InputTextParser;
#[cfg(feature = "tokenizer")]
use jp_parsing::WordItem;

use self::result::{InflectionInformation, WordResult};

use super::query::Form;

/// An in memory Cache for word search results
static SEARCH_CACHE: Lazy<Mutex<SharedCache<u64, WordResult>>> =
    Lazy::new(|| Mutex::new(SharedCache::with_capacity(1000)));

pub(self) struct Search<'a> {
    db: &'a DbPool,
    query: &'a Query,
}

/// Search among all data based on the input query
pub async fn search(db: &DbPool, query: &Query) -> Result<WordResult, Error> {
    let start = SystemTime::now();
    let search = Search { query, db };

    // Try to use cache
    if let Some(c_res) = search.get_cache().await {
        return Ok(c_res);
    }

    // Perform the search
    let results = search.do_search().await?;

    println!("search took {:?}", start.elapsed());
    search.save_cache(results.clone()).await;
    Ok(results)
}

impl<'a> Search<'a> {
    /// Do the search
    async fn do_search(&self) -> Result<WordResult, Error> {
        let (word_results, infl_info) = match self.query.form {
            Form::KanjiReading(_) => kanji::by_reading(self).await?,
            _ => self.do_word_search().await?,
        };

        // Chain and map results into one result vector
        let kanji_results = kanji::load_word_kanji_info(&self, &word_results).await?;
        let kanji_items = kanji_results.len();

        return Ok(WordResult {
            items: Self::merge_words_with_kanji(word_results, kanji_results),
            contains_kanji: kanji_items > 0,
            inflection_info: infl_info,
        });
    }

    /// Search by a word
    async fn do_word_search(&self) -> Result<(Vec<Word>, Option<InflectionInformation>), Error> {
        // Perform searches asynchronously
        let ((native_word_res, infl_info), gloss_word_res): (
            (Vec<Word>, Option<InflectionInformation>),
            Vec<Word>,
        ) = futures::try_join!(self.native_results(), self.gloss_results())?;

        // Chain native and word results into one vector
        Ok((
            native_word_res
                .into_iter()
                .chain(gloss_word_res)
                .collect_vec(),
            infl_info,
        ))
    }

    #[cfg(not(feature = "tokenizer"))]
    fn get_query(&self) -> String {
        self.query.query.clone()
    }

    #[cfg(feature = "tokenizer")]
    async fn get_query<'b>(&'b self) -> Result<(String, Option<WordItem<'static, 'b>>), Error> {
        if !self.query.parse_japanese {
            return Ok((self.query.query.clone(), None));
        }

        let parser =
            InputTextParser::new(&self.db, &self.query.query, &crate::JA_NL_PARSER).await?;

        if let Some(parsed) = parser.parse() {
            if parsed.items.is_empty() {
                return Ok((self.query.query.clone(), None));
            }

            println!("parsed: {:#?}", parsed);
            let index = self.query.word_index.clamp(0, parsed.items.len() - 1);
            let res = parsed.items[index].clone();
            Ok((res.get_lexeme().to_string(), Some(res)))
        } else {
            Ok((self.query.query.clone(), None))
        }
    }

    /// Perform a native word search
    async fn native_results(&self) -> Result<(Vec<Word>, Option<InflectionInformation>), Error> {
        if self.query.language != QueryLang::Japanese {
            return Ok((vec![], None));
        }

        #[cfg(feature = "tokenizer")]
        let (query, morpheme) = self.get_query().await?;

        #[cfg(not(feature = "tokenizer"))]
        let query = self.get_query();

        let query_modified = query != self.query.query;

        #[cfg(feature = "tokenizer")]
        let infl_info = morpheme.as_ref().and_then(|i| {
            (!i.inflections.is_empty()).then(|| InflectionInformation {
                lexeme: i.lexeme.to_owned(),
                forms: i.inflections.clone(),
            })
        });

        #[cfg(not(feature = "tokenizer"))]
        let infl_info: Option<InflectionInformation> = None;

        println!("query: {}", query);

        // Define basic search structure
        let mut word_search = WordSearch::new(self.db, &query);
        word_search
            .with_language(self.query.settings.user_lang)
            .with_english_glosses(self.query.settings.show_english);

        if self.query.has_part_of_speech_tags() {
            word_search.with_pos_filter(&self.query.get_part_of_speech_tags());
        }

        // Perform the word search
        let mut wordresults = if real_string_len(&query) == 1 && query.is_kana() {
            // Search for exact matches only if query.len() <= 2
            let res = word_search
                .with_mode(SearchMode::Exact)
                .with_language(self.query.settings.user_lang)
                .search_native()
                .await?;

            let r = if res.is_empty() {
                // Do another search if no exact result was found
                word_search
                    .with_mode(SearchMode::RightVariable)
                    .search_native()
                    .await?
            } else {
                res
            };

            if query_modified {
                let origi_res = word_search
                    .with_query(&self.query.query)
                    .with_mode(SearchMode::Exact)
                    .search_native()
                    .await?;
                r.into_iter().chain(origi_res).collect()
            } else {
                r
            }
        } else {
            let results = word_search
                .with_mode(SearchMode::RightVariable)
                .search_native()
                .await?;

            // if query was modified search for the
            // original term too
            if query_modified {
                let origi_res = word_search
                    .with_query(&self.query.query)
                    .with_mode(SearchMode::Exact)
                    .search_native()
                    .await?;

                results.into_iter().chain(origi_res).collect()
            } else {
                results
            }
        };

        #[cfg(feature = "tokenizer")]
        let search_order = SearchOrder::new(self.query, &morpheme);

        #[cfg(not(feature = "tokenizer"))]
        let search_order = SearchOrder::new(self.query);

        // Sort the results based
        search_order.sort(&mut wordresults, order::native_search_order);

        // Limit search to 10 results
        wordresults.truncate(10);

        Ok((wordresults, infl_info))
    }

    /// Search gloss readings
    async fn gloss_results(&self) -> Result<Vec<Word>, Error> {
        if !(self.query.language == QueryLang::Foreign
            || self.query.language == QueryLang::Undetected)
        {
            return Ok(vec![]);
        }

        let mode = if real_string_len(&self.query.query) <= 2 {
            SearchMode::Exact
        } else {
            SearchMode::Variable
        };

        // TODO don't make exact search
        let mut word_search = WordSearch::new(self.db, &self.query.query);
        word_search
            .with_language(self.query.settings.user_lang)
            .with_case_insensitivity(true)
            .with_english_glosses(self.query.settings.show_english)
            .with_mode(mode);

        if self.query.has_part_of_speech_tags() {
            word_search.with_pos_filter(&self.query.get_part_of_speech_tags());
        }

        let mut wordresults = word_search.search_by_glosses().await?;

        #[cfg(feature = "tokenizer")]
        let search_order = SearchOrder::new(self.query, &None);

        #[cfg(not(feature = "tokenizer"))]
        let search_order = SearchOrder::new(self.query);

        // Sort the results based
        //GlossWordOrder::new(&self.query.query).sort(&mut wordresults);
        search_order.sort(&mut wordresults, order::foreign_search_order);

        // Limit search to 10 results
        wordresults.truncate(10);

        Ok(wordresults)
    }

    fn merge_words_with_kanji(words: Vec<Word>, kanji: Vec<DbKanji>) -> Vec<Item> {
        kanji
            .into_iter()
            .map(|i| i.into())
            .collect::<Vec<Item>>()
            .into_iter()
            .chain(words.into_iter().map(|i| i.into()).collect_vec())
            .collect_vec()
    }

    async fn get_cache(&self) -> Option<WordResult> {
        SEARCH_CACHE
            .lock()
            .await
            .cache_get(&self.query.get_hash())
            .map(|i| i.clone())
    }

    async fn save_cache(&self, result: WordResult) {
        SEARCH_CACHE
            .lock()
            .await
            .cache_set(self.query.get_hash(), result);
    }
}
