use std::{
    cmp::Ordering,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};

use ::japanese::JapaneseExt;
use actix_web::{
    rt::time::timeout,
    web::{self, Json},
};
use config::Config;
use error::api_error::RestError;
use parse::jmdict::languages::Language;
use query_parser::QueryType;
use search::{
    query::{Query, QueryLang, UserSettings},
    query_parser,
};
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use utils::real_string_len;

/// Request struct for suggestion endpoint
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SuggestionRequest {
    pub input: String,
    #[serde(default)]
    pub lang: String,
}

/// Response struct for suggestion endpoint
#[derive(Clone, Debug, Serialize, Default)]
pub struct SuggestionResponse {
    pub suggestions: Vec<WordPair>,
}

/// a Word with kana and kanji if available
#[derive(Clone, Debug, Serialize, Default)]
pub struct WordPair {
    pub primary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}

/// Max results to show
const MAX_RESULTS: i64 = 10;

/// Get search suggestions
pub async fn suggestion(
    pool: web::Data<Arc<Client>>,
    config: web::Data<Config>,
    payload: Json<SuggestionRequest>,
) -> Result<Json<SuggestionResponse>, actix_web::Error> {
    let query_len = real_string_len(&payload.input);
    if query_len < 1 || query_len > 37 {
        return Err(RestError::BadRequest.into());
    }

    let start = SystemTime::now();

    let mut query_str = payload.input.as_str();

    // Some inputs place the roman letter of the japanese text while typing with romanized input.
    // If input is japanese but last character is a romanized letter, strip it off
    let last_char = query_str.chars().rev().next().unwrap();
    if query_parser::parse_language(query_str) == QueryLang::Japanese
        && last_char.is_roman_letter()
        && query_len > 1
    {
        query_str = &query_str[..query_str.bytes().count() - last_char.len_utf8()];
    }

    // Parse query
    let query = query_parser::QueryParser::new(
        query_str.to_owned(),
        QueryType::Words,
        UserSettings {
            user_lang: Language::from_str(&payload.lang).unwrap_or_default(),
            ..UserSettings::default()
        },
        0,
        0,
    )
    .parse()
    .ok_or(RestError::BadRequest)?;

    let result = timeout(
        Duration::from_millis(config.get_suggestion_timeout()),
        get_suggestion(&pool, query),
    )
    .await
    .map_err(|_| RestError::Timeout)??;

    println!("suggestion took: {:?}", start.elapsed());

    Ok(Json(result))
}

async fn get_suggestion(pool: &Client, query: Query) -> Result<SuggestionResponse, RestError> {
    let suggestions = get_suggestion_by_query(pool, &query).await?;

    if suggestions.suggestions.is_empty() && query.query.is_hiragana() {
        let new_query = Query {
            query: romaji::RomajiExt::to_katakana(query.query.as_str()),
            ..query.clone()
        };
        return Ok(get_suggestion_by_query(pool, &new_query).await?);
    }

    Ok(suggestions)
}

async fn get_suggestion_by_query(
    pool: &Client,
    query: &Query,
) -> Result<SuggestionResponse, RestError> {
    // Get sugesstions for matching language
    let mut word_pairs = match query.language {
        QueryLang::Japanese => japanese::suggestions(&pool, &query.query).await?,
        QueryLang::Foreign | QueryLang::Undetected => {
            foreign::suggestions(&pool, &query, &query.query).await?
        }
    };

    // Put exact matches to top
    word_pairs.sort_by(|a, b| {
        let a_has_reading = a.has_reading(&query.query);
        let b_has_reading = b.has_reading(&query.query);

        if a_has_reading && !b_has_reading {
            Ordering::Less
        } else if b_has_reading && !a_has_reading {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    Ok(SuggestionResponse {
        suggestions: word_pairs,
    })
}

mod japanese {
    use std::iter::FromIterator;

    use futures::{stream::FuturesOrdered, TryStreamExt};
    use itertools::Itertools;

    use super::*;

    /// Get suggestions for foreign search input
    pub(super) async fn suggestions(
        client: &Client,
        query_str: &str,
    ) -> Result<Vec<WordPair>, RestError> {
        get_sequence_ids(client, &query_str).await
    }

    async fn get_sequence_ids(
        client: &Client,
        query_str: &str,
    ) -> Result<Vec<WordPair>, RestError> {
        let seq_query = "SELECT sequence FROM dict WHERE reading LIKE $1 ORDER BY jlpt_lvl DESC NULLS LAST, ARRAY_LENGTH(priorities,1) DESC NULLS LAST, LENGTH(reading) LIMIT $2";

        let rows = client
            .query(
                seq_query,
                &[&format!("{}%", query_str).as_str(), &MAX_RESULTS],
            )
            .await?;

        let mut sequences: Vec<i32> = rows.into_iter().map(|i| i.get(0)).collect();
        sequences.dedup();

        Ok(load_words(&client, &sequences).await?)
    }

    async fn load_words(client: &Client, sequences: &[i32]) -> Result<Vec<WordPair>, RestError> {
        let word_query =
            "select reading, kanji from dict where sequence = $1 and (is_main or kanji = false)";

        let prepared = client.prepare(word_query).await?;

        Ok(FuturesOrdered::from_iter(sequences.into_iter().map(|i| {
            let cloned = prepared.clone();
            async move { client.query(&cloned, &[&i]).await }
        }))
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .filter_map(|word| {
            let words: Vec<(String, bool)> =
                word.into_iter().map(|i| (i.get(0), i.get(1))).collect_vec();

            let kana = words.iter().find(|i| !i.1)?.0.to_owned();
            let kanji = words.iter().find(|i| i.1).map(|i| i.0.to_owned());

            Some(WordPair {
                primary: kana,
                secondary: kanji,
            })
        })
        .collect())
    }
}

mod foreign {
    use super::*;

    /// Get suggestions for foreign search input
    pub async fn suggestions(
        client: &Client,
        query: &Query,
        query_str: &str,
    ) -> Result<Vec<WordPair>, RestError> {
        let lang_str = match query.settings.user_lang {
            Language::English => "language = 0".to_owned(),
            _ => {
                let lang: i32 = query.settings.user_lang.into();
                format!("language = 0 or language = {}", lang)
            }
        };
        let seq_query = format!("SELECT sense.sequence, sense.gloss FROM sense WHERE gloss ilike $1 AND ({}) ORDER BY LENGTH(gloss) limit 50", lang_str);

        //let seq_query = "SELECT sense.sequence, sense.gloss FROM sense JOIN dict ON (dict.sequence = sense.sequence) WHERE gloss &@ $1 AND (language = 1 OR Language = 0) ORDER BY ARRAY_LENGTH(priorities, 1) DESC NULLS LAST LIMIT 50";

        let rows = client
            .query(seq_query.as_str(), &[&format!("{}%", query_str)])
            .await?;

        let mut words: Vec<String> = rows.into_iter().map(|i| i.get(1)).collect();

        // Put exact matches to top
        words.sort_by(|a, b| {
            let a_lv = levenshtein::levenshtein(a, query_str);
            let b_lv = levenshtein::levenshtein(b, query_str);
            a_lv.cmp(&b_lv)
        });

        words.dedup();
        words.truncate(10);

        Ok(words
            .into_iter()
            .map(|i| WordPair {
                primary: i,
                secondary: None,
            })
            .collect())
    }
}

impl WordPair {
    /// Returns true if [`self`] contains [`reading`]
    fn has_reading(&self, reading: &str) -> bool {
        self.primary == reading
            || self
                .secondary
                .as_ref()
                .map(|i| i == reading)
                .unwrap_or_default()
    }
}
