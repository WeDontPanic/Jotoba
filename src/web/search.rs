use actix_web::{web, HttpResponse};
use search::name;
use serde::Deserialize;

use crate::{
    search::{
        self,
        query::Query,
        query_parser::{QueryParser, QueryType},
    },
    templates, DbPool,
};

#[derive(Deserialize, Debug)]
pub struct QueryStruct {
    #[serde(rename = "search")]
    pub query: Option<String>,
    #[serde(rename = "type")]
    pub search_type: Option<QueryType>,
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
        }
    }
}

fn redirect_home() -> HttpResponse {
    HttpResponse::MovedPermanently()
        .header("Location", "/")
        .finish()
}

/// Endpoint to perform a search
pub async fn search(
    pool: web::Data<DbPool>,
    query_data: web::Query<QueryStruct>,
) -> Result<HttpResponse, actix_web::Error> {
    let query_data = query_data.adjust();

    let q_parser = QueryParser::new(
        query_data.query.clone().unwrap_or_default(),
        query_data.search_type.unwrap_or_default(),
    );

    let query = match q_parser.parse() {
        Some(k) => k,
        None => return Ok(redirect_home()),
    };

    println!("{:#?}", query);

    // Perform the requested type of search and render
    // the appropriate template
    match query.type_ {
        QueryType::Kanji => kanji_search(&pool, query).await,
        QueryType::Sentences => sentence_search().await,
        QueryType::Names => name_search(&pool, query).await,
        QueryType::Words => word_search(&pool, query).await,
    }
}

/// Perform a sentence search and
/// render sentence_search tempalte
async fn sentence_search() -> Result<HttpResponse, actix_web::Error> {
    Ok(redirect_home())
}

/// Perform a kanji search and
/// render kanji_details tempalte
async fn kanji_search(
    pool: &web::Data<DbPool>,
    query: Query,
) -> Result<HttpResponse, actix_web::Error> {
    let start = std::time::SystemTime::now();

    let kanji = search::kanji::search(&pool, &query)
        .await
        .unwrap_or_default();

    println!("kanji loading took: {:?}", start.elapsed().unwrap());

    // if not kanji was found,
    // redirect to word search
    if kanji.is_empty() {
        return Ok(HttpResponse::MovedPermanently()
            .header("Location", format!("/search?type=0&search={}", query.query))
            .finish());
    }

    Ok(HttpResponse::Ok().body(render!(
        templates::base,
        Some(&query),
        None,
        Some(kanji),
        None
    )))
}

/// Perform a name search and
/// render name_search tempalte
async fn name_search(
    pool: &web::Data<DbPool>,
    query: Query,
) -> Result<HttpResponse, actix_web::Error> {
    let start = std::time::SystemTime::now();

    let names = name::search(&pool, &query).await.unwrap();

    println!("name search took {:?}", start.elapsed());
    Ok(HttpResponse::Ok().body(render!(
        templates::base,
        Some(&query),
        None,
        None,
        Some(names),
    )))
}

/// Perform a word search and
/// render word_search tempalte
async fn word_search(
    pool: &web::Data<DbPool>,
    query: Query,
) -> Result<HttpResponse, actix_web::Error> {
    // Perform a search
    let result = search::everything::search(&pool, &query).await.unwrap();

    Ok(HttpResponse::Ok().body(render!(
        templates::base,
        Some(&query),
        Some(result),
        None,
        None,
    )))
}
