pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use actix_web::web::Json;
use error::api_error::RestError;
use types::jotoba::languages::Language;
use search::{
    query::{Query, UserSettings},
    query_parser::{QueryParser, QueryType},
};
use serde::Deserialize;

pub type Result<T> = std::result::Result<T, RestError>;

/// An Search API payload
#[derive(Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "query")]
    query_str: String,

    #[serde(default)]
    language: Language,

    #[serde(default)]
    no_english: bool,
}

impl SearchRequest {
    fn parse(payload: Json<SearchRequest>, q_type: QueryType) -> Result<Query> {
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
}
