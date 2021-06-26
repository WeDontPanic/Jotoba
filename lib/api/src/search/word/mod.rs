pub mod response;

use actix_web::web::{Data, Json};
use deadpool_postgres::Pool;
use error::api_error::RestError;
use parse::jmdict::languages::Language;
use search::{
    query::{Query, UserSettings},
    query_parser::{QueryParser, QueryType::Words},
};
use serde::Deserialize;

use self::response::Response;

#[derive(Debug, Deserialize)]
pub struct Request {
    #[serde(rename = "query")]
    query_str: String,

    #[serde(default)]
    language: Language,
}

type Result<T> = std::result::Result<T, RestError>;

impl Request {
    fn parse(payload: Json<Request>) -> Result<Query> {
        let settings = UserSettings {
            user_lang: payload.language,
            ..UserSettings::default()
        };

        let query = QueryParser::new(payload.query_str.clone(), Words, settings, 0, 0)
            .parse()
            .ok_or(RestError::BadRequest)?;

        Ok(query)
    }
}

/// Get kanji by its radicals
pub async fn word_search(payload: Json<Request>, pool: Data<Pool>) -> Result<Json<Response>> {
    let query = Request::parse(payload)?;

    Ok(Json(search::word::search(&pool, &query).await?.into()))
}
