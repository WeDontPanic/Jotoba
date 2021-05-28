use std::{str::FromStr, time::SystemTime};

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{templates, web::BaseData};
use models::DbPool;
use parse::jmdict::languages::Language;
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
    /// Adjusts the search query
    /// Trim and map empty search queries to Option::None
    /// Ensures search_type is always 'Some()'
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
}

/// Endpoint to perform a search
pub async fn search(
    pool: web::Data<DbPool>,
    query_data: web::Query<QueryStruct>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let query_data = query_data.adjust();

    let q_parser = QueryParser::new(
        query_data.query.clone().unwrap_or_default(),
        query_data.search_type.unwrap_or_default(),
        parse_settings(&request),
        query_data.page.unwrap_or_default(),
        query_data.word_index.unwrap_or_default(),
    );

    let query = match q_parser.parse() {
        Some(k) => k,
        None => return Ok(redirect_home()),
    };

    println!("{:#?}", query);

    // Perform the requested type of search and render
    // the appropriate template
    // TODO refactor to return a site to display instead of the http response
    match query.type_ {
        QueryType::Kanji => kanji_search(&pool, query).await,
        QueryType::Sentences => sentence_search(&pool, query).await,
        QueryType::Names => name_search(&pool, query).await,
        QueryType::Words => word_search(&pool, query).await,
    }
}

/// Perform a sentence search and
/// render sentence_search tempalte
async fn sentence_search(
    pool: &web::Data<DbPool>,
    query: Query,
) -> Result<HttpResponse, web_error::Error> {
    let start = SystemTime::now();
    let result = search::sentence::search(&pool, &query).await?;
    println!("sentence searh took: {:?}", start.elapsed());

    let template_data = BaseData::new_sentence_search(&query, result);
    Ok(HttpResponse::Ok().body(render!(templates::base, template_data,)))
}

/// Perform a kanji search and
/// render kanji_details tempalte
async fn kanji_search(
    pool: &web::Data<DbPool>,
    query: Query,
) -> Result<HttpResponse, web_error::Error> {
    let start = std::time::SystemTime::now();

    let kanji = search::kanji::search(&pool, &query).await?;

    println!("kanji loading took: {:?}", start.elapsed().unwrap());

    // if not kanji was found,
    // redirect to word search
    if kanji.is_empty() {
        return Ok(HttpResponse::MovedPermanently()
            .header("Location", format!("/search?type=0&search={}", query.query))
            .finish());
    }

    let template_data = BaseData::new_kanji_search(&query, kanji);
    Ok(HttpResponse::Ok().body(render!(templates::base, template_data)))
}

/// Perform a name search and
/// render name_search tempalte
async fn name_search(
    pool: &web::Data<DbPool>,
    query: Query,
) -> Result<HttpResponse, web_error::Error> {
    let start = std::time::SystemTime::now();

    let names = search::name::search(&pool, &query).await?;

    println!("name search took {:?}", start.elapsed());

    let template_data = BaseData::new_name_search(&query, names);

    Ok(HttpResponse::Ok().body(render!(templates::base, template_data)))
}

/// Perform a word search and
/// render word_search tempalte
async fn word_search(
    pool: &web::Data<DbPool>,
    query: Query,
) -> Result<HttpResponse, web_error::Error> {
    // Perform a search
    let result = search::word::search(&pool, &query).await?;

    let template_data = BaseData::new_word_search(&query, result);
    Ok(HttpResponse::Ok().body(render!(templates::base, template_data)))
}

fn redirect_home() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .header("Location", "/")
        .finish()
}

fn parse_settings(request: &HttpRequest) -> UserSettings {
    let show_english = request
        .cookie("show_english")
        .and_then(|i| i.value().parse().ok())
        .unwrap_or_else(|| UserSettings::default().show_english);

    let user_lang = request
        .cookie("default_lang")
        .and_then(|i| Language::from_str(i.value()).ok())
        .unwrap_or_default();

    let english_on_top = request
        .cookie("show_english_on_top")
        .and_then(|i| i.value().parse().ok())
        .unwrap_or_else(|| UserSettings::default().english_on_top)
        && show_english;

    UserSettings {
        user_lang,
        show_english,
        english_on_top,
        ..Default::default()
    }
}
