pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use actix_web::web::Json;
use error::api_error::RestError;
use search::{
    query::{Query, UserSettings},
    query_parser::QueryParser,
};
use types::{api::search::SearchRequest, jotoba::search::QueryType};

pub type Result<T> = std::result::Result<T, RestError>;

pub(crate) fn parse_query(payload: Json<SearchRequest>, q_type: QueryType) -> Result<Query> {
    let settings = UserSettings {
        user_lang: payload.language,
        show_english: !payload.no_english,
        ..UserSettings::default()
    };

    let query = QueryParser::new(payload.query_str.clone(), q_type, settings, 0, 0, true)
        .parse()
        .ok_or(RestError::BadRequest)?;

    Ok(query)
}
