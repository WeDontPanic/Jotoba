mod kanji;
mod names;
mod request;
mod storage;
mod words;

pub use storage::load_suggestions;

use std::cmp::Ordering;

use config::Config;
use error::api_error::RestError;
use japanese::JapaneseExt;
use search::query::{Form, Query, QueryLang};
use types::api::completions::{Response, WordPair};
use types::{
    api::completions::Request,
    jotoba::{languages::Language, search::QueryType},
};
use utils::bool_ord;

use actix_web::{
    rt::time,
    web::{self, Json},
};

/// Get search suggestions endpoint
pub async fn suggestion_ep(
    config: web::Data<Config>,
    payload: Json<Request>,
) -> Result<Json<Response>, actix_web::Error> {
    request::validate(&payload)?;

    // Adjust payload and parse to query
    let query = request::get_query(&request::adjust(&payload))?;

    // time we allow the suggestion to use in total loaded from the configuration file
    let timeout = config.get_suggestion_timeout();

    let result = time::timeout(timeout, get_suggestions(query))
        .await
        .map_err(|_| RestError::Timeout)??;

    Ok(Json(result))
}

/// Returns best matching suggestions for the given query
async fn get_suggestions(query: Query) -> Result<Response, RestError> {
    match query.type_ {
        QueryType::Sentences | QueryType::Words => {
            if let Some(kanji_reading) = as_kanji_reading(&query) {
                kanji::reading::suggestions(kanji_reading).await
            } else {
                Ok(get_word_suggestions(query).await.unwrap_or_default())
            }
        }
        QueryType::Kanji => kanji_suggestions(query).await,
        QueryType::Names => name_suggestions(query).await,
    }
}

/// Returns name suggestions for the matching input language
async fn name_suggestions(query: Query) -> Result<Response, RestError> {
    Ok(match query.language {
        QueryLang::Japanese => names::native_suggestions(&query).await?,
        QueryLang::Foreign => names::transcription_suggestions(&query).await?,
        _ => Response::default(),
    })
}

/// Returns kanji suggestions
async fn kanji_suggestions(query: Query) -> Result<Response, RestError> {
    if query.language == QueryLang::Foreign {
        kanji::meaning::suggestions(&query).await
    } else {
        Ok(Response::default())
    }
}

/// Returns Some(KanjiReading) if query is or 'could be' a kanji reading query
fn as_kanji_reading(query: &Query) -> Option<types::jotoba::kanji::ReadingSearch> {
    match &query.form {
        Form::KanjiReading(r) => Some(r.clone()),
        _ => {
            let mut query_str = query.original_query.chars();
            let first = query_str.next()?;
            let second = query_str.next()?;

            if first.is_kanji() && second == ' ' {
                Some(types::jotoba::kanji::ReadingSearch {
                    reading: String::new(),
                    literal: first,
                })
            } else {
                None
            }
        }
    }
}

/// Returns word suggestions based on the query. Applies various approaches to give better results
async fn get_word_suggestions(query: Query) -> Option<Response> {
    let response = try_word_suggestions(&query).await?;

    // Tries to do a katakana search if nothing was found
    let result = if response.is_empty() && query.query.is_hiragana() {
        try_word_suggestions(&get_katakana_query(&query)).await?
    } else {
        response
    };

    Some(Response {
        suggestions: result,
        ..Default::default()
    })
}

/// Returns Ok(suggestions) for the given query ordered and ready to display
async fn try_word_suggestions(query: &Query) -> Option<Vec<WordPair>> {
    // Get sugesstions for matching language
    let word_pairs = match query.language {
        QueryLang::Japanese => words::native::suggestions(&query.query)?,
        QueryLang::Foreign | QueryLang::Undetected | QueryLang::Korean => {
            let mut res = words::foreign::suggestions(&query, &query.query)
                .await
                .unwrap_or_default();

            // Order: put exact matches to top
            res.sort_by(|a, b| word_pair_order(a, b, &query.query));
            res
        }
    };

    Some(word_pairs)
}

/// Ordering for [`WordPair`]s which puts the exact matches to top
fn word_pair_order(a: &WordPair, b: &WordPair, query: &str) -> Ordering {
    bool_ord(a.has_reading(&query), b.has_reading(&query))
}

/// Returns an equivalent katakana query
fn get_katakana_query(query: &Query) -> Query {
    Query {
        query: romaji::RomajiExt::to_katakana(query.query.as_str()),
        ..query.clone()
    }
}
