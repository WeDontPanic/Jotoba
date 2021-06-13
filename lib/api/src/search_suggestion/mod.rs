mod foreign;
mod native;
mod storage;

pub use storage::load_suggestions;
use storage::SuggestionItem;

use std::{cmp::Ordering, str::FromStr, sync::Arc};

use config::Config;
use error::api_error::RestError;
use japanese::JapaneseExt;
use parse::jmdict::languages::Language;
use query_parser::{QueryParser, QueryType};
use search::{
    query::{Query, QueryLang, UserSettings},
    query_parser,
    suggestions::SuggestionSearch,
};
use utils::real_string_len;

use actix_web::{
    rt::time,
    web::{self, Json},
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

/// In-memory storage for suggestions
static SUGGESTIONS: OnceCell<SuggestionSearch<Vec<SuggestionItem>>> = OnceCell::new();

/// Request struct for suggestion endpoint
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Request {
    /// The search query to find suggestions for
    pub input: String,

    /// The user configured language
    #[serde(default)]
    pub lang: String,
}

impl Request {
    /// Adjust the query and returns a newly allocated one
    fn adjust(&self) -> Self {
        let mut query_str = self.input.as_str();
        let query_len = real_string_len(&self.input);

        // Some inputs place the roman letter of the japanese text while typing with romanized input.
        // If input is japanese but last character is a romanized letter, strip it off
        let last_char = query_str.chars().rev().next().unwrap();
        if query_parser::parse_language(query_str) == QueryLang::Japanese
            && last_char.is_roman_letter()
            && query_len > 1
        {
            query_str = &query_str[..query_str.bytes().count() - last_char.len_utf8()];
        }

        Self {
            input: query_str.to_owned(),
            lang: self.lang.to_owned(),
        }
    }

    // Returns a [`Query`] based on the [`Request`]
    fn get_query(&self) -> Result<Query, RestError> {
        let query_str = self.input.clone();

        // Doesn't matter here
        let search_type = QueryType::Words;

        let settings = UserSettings {
            user_lang: self.get_language(),
            ..UserSettings::default()
        };

        // Build and parse the query
        let query = QueryParser::new(query_str, search_type, settings, 0, 0)
            .parse()
            .ok_or(RestError::BadRequest)?;

        Ok(query)
    }

    // Returns the user configured language of the [`Request`]
    fn get_language(&self) -> Language {
        Language::from_str(&self.lang).unwrap_or_default()
    }
}

/// Response struct for suggestion endpoint
#[derive(Clone, Debug, Serialize, Default)]
pub struct Response {
    pub suggestions: Vec<WordPair>,
}

/// A word with kana and kanji reading used within [`SuggestionResponse`]
#[derive(Clone, Debug, Serialize, Default, PartialEq)]
pub struct WordPair {
    pub primary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
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

/// Get search suggestions endpoint
pub async fn suggestion_ep(
    pool: web::Data<Arc<Client>>,
    config: web::Data<Config>,
    payload: Json<Request>,
) -> Result<Json<Response>, actix_web::Error> {
    validate_request(&payload)?;

    // Adjust payload and parse to query
    let query = payload.adjust().get_query()?;

    // time we allow the suggestion to use in total loaded from the configuration file
    let timeout = config.get_suggestion_timeout();

    let result = time::timeout(timeout, get_suggestions(&pool, query))
        .await
        .map_err(|_| RestError::Timeout)??;

    Ok(Json(result))
}

/// Returns suggestions based on the query. Applies various approaches to give better results
async fn get_suggestions(pool: &Client, query: Query) -> Result<Response, RestError> {
    let response = get_suggestion_by_query(pool, &query).await?;

    // Tries to do a katakana search if nothing was found
    let result = if response.suggestions.is_empty() && query.query.is_hiragana() {
        get_suggestion_by_query(pool, &get_katakana_query(&query)).await?
    } else {
        response
    };

    Ok(result)
}

/// Returns Ok(suggestions) for the given query ordered and ready to display
async fn get_suggestion_by_query(pool: &Client, query: &Query) -> Result<Response, RestError> {
    // Get sugesstions for matching language
    let mut word_pairs = match query.language {
        QueryLang::Japanese => native::suggestions(&pool, &query.query).await?,
        QueryLang::Foreign | QueryLang::Undetected => foreign::suggestions(&query, &query.query)
            .await
            .unwrap_or_default(),
    };

    // Order: put exact matches to top
    word_pairs.sort_by(|a, b| word_pair_order(a, b, &query.query));

    Ok(Response {
        suggestions: word_pairs,
    })
}

/// Ordering for [`WordPair`]s which puts the exact matches to top
fn word_pair_order(a: &WordPair, b: &WordPair, query: &str) -> Ordering {
    let a_has_reading = a.has_reading(&query);
    let b_has_reading = b.has_reading(&query);

    if a_has_reading && !b_has_reading {
        Ordering::Less
    } else if b_has_reading && !a_has_reading {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

/// Returns an equivalent katakana query
fn get_katakana_query(query: &Query) -> Query {
    Query {
        query: romaji::RomajiExt::to_katakana(query.query.as_str()),
        ..query.clone()
    }
}

/// Validates the API request payload
fn validate_request(payload: &Request) -> Result<(), RestError> {
    let query_len = real_string_len(&payload.input);
    if query_len < 1 || query_len > 37 {
        return Err(RestError::BadRequest.into());
    }

    Ok(())
}
