mod kanji;
mod names;
mod request;
mod words;

use actix_web::web::Json;
use japanese::JapaneseExt;
use search::query::{Form, Query};
use types::{
    api::completions::{Request, Response, WordPair},
    jotoba::{kanji::reading::ReadingSearch, search::QueryType},
};

/// Get search suggestions endpoint
pub async fn suggestion_ep(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    request::validate(&payload)?;

    // Adjust payload and parse to query
    let (query, radicals) = request::get_query(request::adjust(payload.into_inner()))?;

    Ok(Json(get_suggestions(query, radicals)))
}

/// Returns best matching suggestions for the given query
fn get_suggestions(query: Query, radicals: Vec<char>) -> Response {
    let res = match query.type_ {
        QueryType::Kanji => kanji::suggestions(query),
        QueryType::Names => names::suggestions(query),
        QueryType::Words | QueryType::Sentences => {
            if let Some(kanji_reading) = as_kanji_reading(&query) {
                kanji::reading::suggestions(kanji_reading)
            } else {
                words::suggestions(query, &radicals)
            }
        }
    };

    res.unwrap_or_default()
}

/// Returns Some(KanjiReading) if query is or 'could be' a kanji reading query.
/// "Could be" means that a kanji-reading search is being types. This the case
/// if a single kanji and a space is written in the current query
fn as_kanji_reading(query: &Query) -> Option<ReadingSearch> {
    match &query.form {
        Form::KanjiReading(r) => Some(r.clone()),
        _ => {
            let mut query_str = query.original_query.chars();
            let first = query_str.next()?;
            let second = query_str.next()?;

            if first.is_kanji() && second == ' ' {
                Some(ReadingSearch {
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
        .collect()
}
