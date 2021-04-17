use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{templates, DbPool};

#[derive(Deserialize, Debug)]
pub struct QueryStruct {
    #[serde(rename = "search")]
    query: Option<String>,
    #[serde(rename = "type")]
    search_type: Option<QueryType>,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub enum QueryType {
    #[serde(rename = "1")]
    Sentences,
    #[serde(rename = "2")]
    Names,
    #[serde(rename = "0", other)]
    WordsAndKanji,
}

impl Default for QueryType {
    fn default() -> Self {
        Self::WordsAndKanji
    }
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

/// Endpoint to perform a search
pub async fn search(
    pool: web::Data<DbPool>,
    query_data: web::Query<QueryStruct>,
) -> Result<HttpResponse, actix_web::Error> {
    let query_data = query_data.adjust();

    let search_query = match query_data.query {
        Some(s) => s,
        None => {
            return Ok(HttpResponse::MovedPermanently()
                .header("Location", "/")
                .finish())
        }
    };

    // Perform a search
    let result = crate::search::everything::search(&pool, &search_query)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(render!(
        templates::base,
        Some(search_query.clone()),
        Some(result)
    )))
}
