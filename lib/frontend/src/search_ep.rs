use std::str::FromStr;

use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{templates, BaseData};
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
    /// Adjusts the search query trim and map empty search queries to Option::None
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
    fn as_query_parser(&self, request: &HttpRequest) -> QueryParser {
        QueryParser::new(
            self.query.clone().unwrap_or_default(),
            self.search_type.unwrap_or_default(),
            parse_settings(&request),
            self.page.unwrap_or_default(),
            self.word_index.unwrap_or_default(),
        )
    }
}

/// Endpoint to perform a search
pub async fn search(
    pool: web::Data<DbPool>,
    query_data: web::Query<QueryStruct>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let query_data = query_data.adjust();

    let q_parser = query_data.as_query_parser(&request);

    let query = match q_parser.parse() {
        Some(k) => k,
        None => return Ok(redirect_home()),
    };

    println!("{:#?}", query);

    // Perform the requested type of search and return base-data to display
    let site_data = match query.type_ {
        QueryType::Kanji => kanji_search(&pool, &query).await,
        QueryType::Sentences => sentence_search(&pool, &query).await,
        QueryType::Names => name_search(&pool, &query).await,
        QueryType::Words => word_search(&pool, &query).await,
    }?;

    Ok(HttpResponse::Ok().body(render!(templates::base, site_data)))
}

/// Perform a sentence search
async fn sentence_search<'a>(
    pool: &web::Data<DbPool>,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let result = search::sentence::search(&pool, &query).await?;
    Ok(BaseData::new_sentence_search(&query, result))
}

/// Perform a kanji search
async fn kanji_search<'a>(
    pool: &web::Data<DbPool>,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let kanji = search::kanji::search(&pool, &query).await?;
    Ok(BaseData::new_kanji_search(&query, kanji))
}

/// Perform a name search
async fn name_search<'a>(
    pool: &web::Data<DbPool>,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let names = search::name::search(&pool, &query).await?;
    Ok(BaseData::new_name_search(&query, names))
}

/// Perform a word search
async fn word_search<'a>(
    pool: &web::Data<DbPool>,
    query: &'a Query,
) -> Result<BaseData<'a>, web_error::Error> {
    let result = search::word::search(&pool, &query).await?;
    Ok(BaseData::new_word_search(&query, result))
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
