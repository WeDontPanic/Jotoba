pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use actix_web::web::Json;
use error::api_error::RestError;
use search::query::{parser::QueryParser, Query, UserSettings};
use types::{api::search::SearchRequest, jotoba::search::SearchTarget};

pub type Result<T> = std::result::Result<T, RestError>;

pub(crate) fn parse_query(payload: Json<SearchRequest>, q_type: SearchTarget) -> Result<Query> {
    let settings = UserSettings {
        user_lang: payload.language,
        show_english: !payload.no_english,
        ..UserSettings::default()
    };

    let q_str = payload.query_str.clone();

    let query = QueryParser::new(q_str, q_type, settings)
        .parse()
        .ok_or(RestError::BadRequest)?;

    Ok(query)
}
