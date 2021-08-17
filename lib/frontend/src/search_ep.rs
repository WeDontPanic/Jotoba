use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use super::user_settings;

use actix_web::{rt::time::timeout, web, HttpRequest, HttpResponse};
use localization::TranslationDict;
use percent_encoding::percent_decode;
use serde::Deserialize;

use crate::{templates, BaseData};
use config::Config;
use search::{
    self,
    query::{Query, UserSettings},
    query_parser::{QueryParser, QueryType},
};

use super::web_error;

#[derive(Deserialize, Debug)]
pub struct QueryStruct {
    #[serde(rename = "t")]
    pub search_type: Option<QueryType>,
    #[serde(rename = "i")]
    pub word_index: Option<usize>,
    #[serde(rename = "page")]
    pub page: Option<usize>,

    #[serde(skip_serializing, skip_deserializing)]
    pub query_str: String,
}

impl QueryStruct {
    /// Adjusts the search query trim and map empty search queries to Option::None.
    /// Ensures `search_type` is always 'Some()'
    fn adjust(&self, query_str: String) -> Self {
        let query_str = query_str.trim().to_string();

        QueryStruct {
            query_str,
            search_type: Some(self.search_type.unwrap_or_default()),
            page: self.page,
            word_index: self.word_index,
        }
    }

    /// Returns a [`QueryParser`] of the query
    fn as_query_parser(&self, user_settings: UserSettings) -> QueryParser {
        QueryParser::new(
            self.query_str.clone(),
            self.search_type.unwrap_or_default(),
            user_settings,
            self.page.unwrap_or_default(),
            self.word_index.unwrap_or_default(),
            true,
        )
    }
}

/// Endpoint to perform a search
pub async fn search(
    query: web::Path<String>,
    query_data: web::Query<QueryStruct>,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let settings = user_settings::parse(&request);

    let query = percent_decode(query.as_bytes()).decode_utf8()?;

    //session::init(&session, &settings);

    // Parse query and redirect to home on error
    let query = match query_data
        .adjust(query.to_string())
        .as_query_parser(settings)
        .parse()
    {
        Some(k) => k,
        None => return Ok(redirect_home()),
    };

    let start = SystemTime::now();

    // Perform the requested type of search and return base-data to display
    let search_duration = start.elapsed();

    // Log search duration if too long and available
    let search_result = timeout(
        config.get_search_timeout(),
        do_search(query.type_, &locale_dict, settings, &query),
    )
    .await
    .map_err(|_| {
        report_timeout(&request, &query);
        web_error::Error::SearchTimeout
    })??;

    if let Ok(search_duration) = search_duration {
        if search_duration > config.get_query_report_timeout() {
            log_duration(query.type_, search_duration);
        }
    }

    Ok(HttpResponse::Ok().body(render!(templates::base, search_result)))
}

/// Run the search and return the `BaseData` for the result page to render
async fn do_search<'a>(
    querytype_: QueryType,
    locale_dict: &'a TranslationDict,
    settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    match querytype_ {
        QueryType::Kanji => kanji_search(&locale_dict, settings, &query).await,
        QueryType::Sentences => sentence_search(&locale_dict, settings, &query).await,
        QueryType::Names => name_search(&locale_dict, settings, &query).await,
        QueryType::Words => word_search(&locale_dict, settings, &query).await,
    }
}

#[cfg(not(feature = "sentry_error"))]
fn log_duration(search_type: QueryType, duration: Duration) {
    use log::warn;
    warn!("{:?}-search took: {:?}", search_type, duration);
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
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let result = search::sentence::search(&query).await?;
    Ok(BaseData::new_sentence_search(
        &query,
        result,
        locale_dict,
        user_settings,
    ))
}

/// Perform a kanji search
async fn kanji_search<'a>(
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let kanji = search::kanji::search(&query).await?;
    Ok(BaseData::new_kanji_search(
        &query,
        kanji,
        locale_dict,
        user_settings,
    ))
}

/// Perform a name search
async fn name_search<'a>(
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let names = search::name::search(&query).await?;
    Ok(BaseData::new_name_search(
        &query,
        names,
        locale_dict,
        user_settings,
    ))
}

/// Perform a word search
async fn word_search<'a>(
    locale_dict: &'a TranslationDict,
    user_settings: UserSettings,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let result = search::word::search(&query).await?;
    Ok(BaseData::new_word_search(
        &query,
        result,
        locale_dict,
        user_settings,
    ))
}

fn redirect_home() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .append_header(("Location", "/"))
        .finish()
}

/// Reports a search timeout to sentry
#[cfg(not(feature = "sentry_error"))]
fn report_timeout(_request: &HttpRequest, query: &Query) {
    let msg = format!("{:?}-search \"{}\" timed out", query.type_, query.query);
    log::error!("{}", msg);
}

/// Reports a search timeout to sentry
#[cfg(feature = "sentry_error")]
fn report_timeout(request: &HttpRequest, query: &Query) {
    use sentry::{protocol::Event, Level};
    let msg = format!("{:?}-search \"{}\" timed out", query.type_, query.query);
    sentry::capture_event(Event {
        request: Some(sentry_request_from_http(request)),
        level: Level::Error,
        message: Some(msg),
        ..Default::default()
    });
}

/// Build a Sentry request struct from the HTTP request
#[cfg(feature = "sentry_error")]
fn sentry_request_from_http(request: &HttpRequest) -> sentry::protocol::Request {
    use sentry::protocol::Request;

    let sentry_req = Request {
        url: format!(
            "{}://{}{}",
            request.connection_info().scheme(),
            request.connection_info().host(),
            request.uri()
        )
        .parse()
        .ok(),
        method: Some(request.method().to_string()),
        headers: request
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or_default().to_string()))
            .collect(),
        ..Default::default()
    };

    sentry_req
}
