use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use super::user_settings;

use actix_web::{rt::time::timeout, web, HttpRequest, HttpResponse};
use localization::TranslationDict;
use percent_encoding::percent_decode;

use crate::{templates, url_query::QueryStruct, BaseData, ResultData};
use config::Config;
use search::{
    self,
    query::{Query, UserSettings},
    query_parser::QueryType,
};

use super::web_error;

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

    let search_timeout = config.get_search_timeout();

    // Log search duration if too long and available
    let search_result = timeout(
        search_timeout,
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
    let mut base_data = BaseData::new(locale_dict, settings);
    let result_data = match querytype_ {
        QueryType::Kanji => kanji_search(&query).await,
        QueryType::Sentences => sentence_search(&query).await,
        QueryType::Names => name_search(&mut base_data, &query).await,
        QueryType::Words => word_search(&mut base_data, &query).await,
    }?;

    Ok(base_data.with_search_result(query, result_data))
}

type SResult = Result<ResultData, web_error::Error>;

/// Perform a sentence search
async fn sentence_search<'a>(query: &'a Query) -> SResult {
    let result = search::sentence::search(&query).await?;
    Ok(ResultData::Sentence(result))
}

/// Perform a kanji search
async fn kanji_search<'a>(query: &'a Query) -> SResult {
    let result = search::kanji::search(&query).await?;
    Ok(ResultData::KanjiInfo(result))
}

/// Perform a name search
async fn name_search<'a>(base_data: &mut BaseData<'a>, query: &'a Query) -> SResult {
    let result = search::name::search(&query).await?;
    base_data.with_pages(result.total_count, query.page as u32);
    Ok(ResultData::Name(result.items))
}

/// Perform a word search
async fn word_search<'a>(base_data: &mut BaseData<'a>, query: &'a Query) -> SResult {
    let result = search::word::search(&query).await?;
    base_data.with_pages(result.count as u32, query.page as u32);
    Ok(ResultData::Word(result))
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

fn redirect_home() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .append_header(("Location", "/"))
        .finish()
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
