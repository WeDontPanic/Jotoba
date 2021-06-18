use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use super::user_settings;

use actix_web::{web, HttpRequest, HttpResponse};
use deadpool_postgres::Pool;
use localization::TranslationDict;
use serde::Deserialize;

use crate::{session, templates, BaseData};
use config::Config;
use models::{DbConnection, DbPool};
use search::{
    self,
    query::{Query, UserSettings},
    query_parser::{QueryParser, QueryType},
};

use super::web_error;

#[derive(Deserialize, Debug)]
pub struct QueryStruct {
    #[serde(rename = "search")]
    pub query: Option<String>,
    #[serde(rename = "type")]
    pub search_type: Option<QueryType>,
    #[serde(rename = "word_index")]
    pub word_index: Option<usize>,
    #[serde(rename = "page")]
    pub page: Option<usize>,
}

impl QueryStruct {
    /// Adjusts the search query trim and map empty search queries to Option::None.
    /// Ensures `search_type` is always 'Some()'
    fn adjust(&self) -> Self {
        let search_query = self
            .query
            .clone()
            .map(|i| i.trim().to_string())
            .and_then(|i| (!i.is_empty()).then(|| i));

        QueryStruct {
            query: search_query,
            search_type: Some(self.search_type.unwrap_or_default()),
            page: self.page,
            word_index: self.word_index,
        }
    }

    /// Returns a [`QueryParser`] of the query
    fn as_query_parser(&self, user_settings: UserSettings) -> QueryParser {
        QueryParser::new(
            self.query.clone().unwrap_or_default(),
            self.search_type.unwrap_or_default(),
            user_settings,
            self.page.unwrap_or_default(),
            self.word_index.unwrap_or_default(),
        )
    }
}

/// Endpoint to perform a search
pub async fn search(
    pool: web::Data<DbPool>,
    poolv2: web::Data<Pool>,
    query_data: web::Query<QueryStruct>,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let pool = pool.get().unwrap();
    let settings = user_settings::parse(&request);

    //session::init(&session, &settings);

    // Parse query and redirect to home on error
    let query = match query_data.adjust().as_query_parser(settings).parse() {
        Some(k) => k,
        None => return Ok(redirect_home()),
    };

    let start = SystemTime::now();

    // Perform the requested type of search and return base-data to display
    let site_data = match query.type_ {
        QueryType::Kanji => kanji_search(&pool, &locale_dict, settings, &query).await,
        QueryType::Sentences => sentence_search(&poolv2, &locale_dict, settings, &query).await,
        QueryType::Names => name_search(&poolv2, &locale_dict, settings, &query).await,
        QueryType::Words => word_search(&pool, &locale_dict, settings, &query).await,
    }?;
    let search_duration = start.elapsed();

    // Log search duration if too long and available
    if let Ok(search_duration) = search_duration {
        if search_duration > config.get_query_report_timeout() {
            log_duration(query.type_, search_duration);
        }
    }

    Ok(HttpResponse::Ok().body(render!(templates::base, site_data)))
}

#[cfg(not(feature = "sentry_error"))]
fn log_duration(search_type: QueryType, duration: Duration) {
    use log::warn;
    warn!("Search took: {:?}", duration);
}

#[cfg(feature = "sentry_error")]
fn log_duration(search_type: QueryType, duration: Duration) {
    sentry::capture_message(
        format!("{:?}-search took: {:?}", search_type, duration).as_str(),
        sentry::Level::Warning,
    );
}

/// Perform a sentence search
async fn sentence_search<'a>(
    pool: &Pool,
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let result = search::sentence::search(&pool, &query).await?;
    Ok(BaseData::new_sentence_search(
        &query,
        result,
        locale_dict,
        user_settings,
    ))
}

/// Perform a kanji search
async fn kanji_search<'a>(
    pool: &DbConnection,
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let kanji = search::kanji::search(&pool, &query).await?;
    Ok(BaseData::new_kanji_search(
        &query,
        kanji,
        locale_dict,
        user_settings,
    ))
}

/// Perform a name search
async fn name_search<'a>(
    pool: &Pool,
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let names = search::name::search(&pool, &query).await?;
    Ok(BaseData::new_name_search(
        &query,
        names,
        locale_dict,
        user_settings,
    ))
}

/// Perform a word search
async fn word_search<'a>(
    pool: &DbConnection,
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let result = search::word::search(&pool, &query).await?;
    Ok(BaseData::new_word_search(
        &query,
        result,
        locale_dict,
        user_settings,
    ))
}

fn redirect_home() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .header("Location", "/")
        .finish()
}
