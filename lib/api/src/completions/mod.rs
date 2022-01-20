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
use search::query::{Form, Query};
use types::api::completions::{Response, WordPair};
use types::{
    api::completions::Request,
    jotoba::{languages::Language, search::QueryType},
};

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
    let query = request::get_query(&request::adjust(payload.into_inner()))?;

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
                Ok(words::suggestions(query).await.unwrap_or_default())
            }
        }
        QueryType::Kanji => kanji::suggestions(query).await,
        QueryType::Names => names::suggestions(query).await,
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
