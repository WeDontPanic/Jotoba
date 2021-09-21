use std::str::FromStr;

use error::api_error::RestError;
use japanese::JapaneseExt;
use resources::parse::jmdict::languages::Language;
use search::{
    query::{Query, QueryLang, UserSettings},
    query_parser::{self, QueryParser, QueryType},
};
use serde::Deserialize;
use utils::real_string_len;

/// Request payload structure for suggestion endpoint
#[derive(Deserialize)]
pub struct Request {
    /// The search query to find suggestions for
    pub input: String,

    /// The user configured language
    #[serde(default)]
    pub lang: String,

    /// The search type the input is designed for
    #[serde(default)]
    pub search_type: QueryType,
}

impl Request {
    /// Adjust the query and returns a newly allocated one
    pub(crate) fn adjust(&self) -> Self {
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
            search_type: self.search_type,
        }
    }

    /// Returns a `Query` based on the `Request`
    pub(crate) fn get_query(&self) -> Result<Query, RestError> {
        let query_str = self.input.trim_start().to_string();

        let search_type = self.search_type;

        let settings = UserSettings {
            user_lang: self.get_language(),
            ..UserSettings::default()
        };

        // Build and parse the query
        let query = QueryParser::new(query_str, search_type, settings, 0, 0, false)
            .parse()
            .ok_or(RestError::BadRequest)?;

        Ok(query)
    }

    /// Returns the user configured language of the [`Request`]
    #[inline]
    pub(crate) fn get_language(&self) -> Language {
        Language::from_str(&self.lang).unwrap_or_default()
    }
}

/// Validates the API request payload
pub(crate) fn validate(payload: &Request) -> Result<(), RestError> {
    let query_len = real_string_len(&payload.input);
    if query_len < 1 || query_len > 37 {
        return Err(RestError::BadRequest.into());
    }

    Ok(())
}
