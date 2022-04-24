mod kanji;
mod names;
mod request;
mod storage;
mod words;

pub use storage::load_suggestions;

use error::api_error::RestError;
use japanese::JapaneseExt;
use search::query::{Form, Query};
use types::{
    api::completions::{Request, Response, WordPair},
    jotoba::search::QueryType,
};

use actix_web::web::Json;

/// Get search suggestions endpoint
pub async fn suggestion_ep(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    request::validate(&payload)?;

    // Adjust payload and parse to query
    let (query, radicals) = request::get_query(request::adjust(payload.into_inner()))?;

    let result = get_suggestions(query, radicals)?;

    Ok(Json(result))
}

/// Returns best matching suggestions for the given query
fn get_suggestions(query: Query, radicals: Vec<char>) -> Result<Response, RestError> {
    match query.type_ {
        QueryType::Sentences | QueryType::Words => {
            if let Some(kanji_reading) = as_kanji_reading(&query) {
                kanji::reading::suggestions(kanji_reading)
            } else {
                Ok(words::suggestions(query, &radicals).unwrap_or_default())
            }
        }
        QueryType::Kanji => kanji::suggestions(query),
        QueryType::Names => names::suggestions(query),
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

/// Converts engine output to a set of `WordPair`
#[inline]
pub(crate) fn convert_results(engine_output: Vec<autocompletion::index::Output>) -> Vec<WordPair> {
    engine_output
        .into_iter()
        .map(|i| WordPair {
            primary: i.primary,
            secondary: i.secondary,
        })
        .collect::<Vec<_>>()
}
