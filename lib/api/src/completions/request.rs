use std::str::FromStr;

use error::api_error::RestError;
use japanese::JapaneseExt;
use search::{
    query::{Query, QueryLang, UserSettings},
    query_parser::{self, QueryParser},
};
use types::api::completions::Request;
use types::jotoba::languages::Language;
use utils::real_string_len;

/// Adjust the query and returns a newly allocated one
pub(crate) fn adjust(request: &Request) -> Request {
    let mut query_str = request.input.as_str();
    let query_len = real_string_len(&request.input);

    // Some inputs place the roman letter of the japanese text while typing with romanized input.
    // If input is japanese but last character is a romanized letter, strip it off

    let last_chars = query_str.chars().rev().take(2).collect::<Vec<_>>();
    if query_parser::parse_language(query_str) == QueryLang::Japanese
        && !last_chars
            .iter()
            .any(|i| !i.is_roman_letter() && !i.is_japanese())
        && query_len > 1
        && !last_chars.is_empty()
    {
        let len: usize = last_chars
            .into_iter()
            .filter(|i| i.is_roman_letter())
            .map(|i| i.len_utf8())
            .sum();
        query_str = &query_str[..query_str.len() - len];
    }

    Request {
        input: query_str.to_owned(),
        lang: request.lang.to_owned(),
        search_type: request.search_type,
    }
}

/// Returns a `Query` based on the `Request`
pub(crate) fn get_query(request: &Request) -> Result<Query, RestError> {
    let query_str = request.input.trim_start().to_string();

    let search_type = request.search_type;

    let settings = UserSettings {
        user_lang: get_language(request),
        ..UserSettings::default()
    };

    // Build and parse the query
    let query = QueryParser::new(query_str, search_type, settings, 0, 0, false, None)
        .parse()
        .ok_or(RestError::BadRequest)?;

    Ok(query)
}

/// Returns the user configured language of the [`Request`]
#[inline]
pub(crate) fn get_language(request: &Request) -> Language {
    Language::from_str(&request.lang).unwrap_or_default()
}

/// Validates the API request payload
pub(crate) fn validate(payload: &Request) -> Result<(), RestError> {
    let query_len = real_string_len(&payload.input);
    if query_len < 1 || query_len > 37 {
        return Err(RestError::BadRequest.into());
    }

    Ok(())
}
