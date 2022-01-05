use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use super::user_settings;

use actix_web::{rt::time::timeout, web, HttpRequest, HttpResponse};
use localization::TranslationDict;
use percent_encoding::percent_decode;
use types::jotoba::search::QueryType;

use crate::{
    templates,
    url_query::{NoJSQueryStruct, QueryStruct},
    BaseData, ResultData, SearchHelp,
};
use config::Config;
use search::{
    self,
    query::{Query, UserSettings},
};

use super::web_error;

/// Endpoint to perform a search
pub async fn search_ep_no_js(
    query_data: web::Query<NoJSQueryStruct>,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let (query_data, query) = query_data.0.to_query_struct();
    search(query, query_data, locale_dict, config, request).await
}

/// Endpoint to perform a search
pub async fn search_ep(
    query: web::Path<String>,
    query_data: web::Query<QueryStruct>,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let query = percent_decode(query.as_bytes()).decode_utf8()?.to_string();
    search(query, query_data.0, locale_dict, config, request).await
}

async fn search(
    query: String,
    query_data: QueryStruct,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let settings = user_settings::parse(&request);

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
        do_search(query.type_, &locale_dict, settings, &query, &config),
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

    Ok(HttpResponse::Ok().body(render!(templates::base, search_result).render()))
}

/// Run the search and return the `BaseData` for the result page to render
async fn do_search<'a>(
    querytype: QueryType,
    locale_dict: &'a TranslationDict,
    settings: UserSettings,
    query: &'a Query,
    config: &'a Config,
) -> Result<BaseData<'a>, web_error::Error> {
    let mut base_data = BaseData::new(locale_dict, settings, &config.asset_hash, &config);

    let result_data = match querytype {
        QueryType::Kanji => kanji_search(&mut base_data, &query).await,
        QueryType::Sentences => sentence_search(&mut base_data, &query).await,
        QueryType::Names => name_search(&mut base_data, &query).await,
        QueryType::Words => word_search(&mut base_data, &query).await,
    }?;

    let mut search_help: Option<SearchHelp> = None;
    if result_data.is_empty() {
        let query = query.to_owned();
        search_help = web::block(move || build_search_help(querytype, &query)).await?;
    }

    Ok(base_data.with_search_result(query, result_data, search_help))
}

type SResult = Result<ResultData, web_error::Error>;

/// Perform a sentence search
async fn sentence_search<'a>(base_data: &mut BaseData<'a>, query: &'a Query) -> SResult {
    let q = query.to_owned();
    let result = web::block(move || search::sentence::search(&q)).await??;

    base_data.with_pages(result.len as u32, query.page as u32);
    Ok(ResultData::Sentence(result))
}

/// Perform a kanji search
async fn kanji_search<'a>(base_data: &mut BaseData<'a>, query: &'a Query) -> SResult {
    let q = query.to_owned();
    let result = web::block(move || search::kanji::search(&q)).await??;
    base_data.with_cust_pages(
        result.total_items as u32,
        query.page as u32,
        query.settings.kanji_page_size,
        400,
    );
    Ok(ResultData::KanjiInfo(result.items))
}

/// Perform a name search
async fn name_search<'a>(base_data: &mut BaseData<'a>, query: &'a Query) -> SResult {
    let q = query.to_owned();
    let result = web::block(move || search::name::search(&q)).await??;

    base_data.with_pages(result.total_count, query.page as u32);
    Ok(ResultData::Name(result.items))
}

/// Perform a word search
async fn word_search<'a>(base_data: &mut BaseData<'a>, query: &'a Query) -> SResult {
    let q = query.to_owned();
    let result = web::block(move || search::word::search(&q)).await??;

    base_data.with_pages(result.count as u32, query.page as u32);
    Ok(ResultData::Word(result))
}

/// Build a [`SearchHelp`] in for cases without any search results
fn build_search_help(querytype: QueryType, query: &Query) -> Option<SearchHelp> {
    let mut help = SearchHelp::default();

    for qt in QueryType::iterate().filter(|i| *i != querytype) {
        match qt {
            QueryType::Kanji => help.kanji = search::kanji::guess_result(query),
            QueryType::Sentences => help.sentences = search::sentence::guess_result(query),
            QueryType::Names => help.names = search::name::guess_result(query),
            QueryType::Words => help.words = search::word::guess_result(query),
        }
    }

    if querytype == QueryType::Words {
        help.other_langs = search::word::guess_inp_language(query);
    }

    (!help.is_empty()).then(|| help)
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
