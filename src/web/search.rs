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
    #[serde(rename = "0")]
    WordsAndKanji,
    #[serde(rename = "1")]
    Sentences,
    #[serde(rename = "2")]
    Names,
    #[serde(other)]
    Other,
}

impl QueryStruct {
    /// Adjusts the search query
    /// Trim and map empty search queries to Option::None
    /// Map QueryType::Other to Option::None
    fn adjust(&self) -> Self {
        let search_query = self
            .query
            .clone()
            .map(|i| i.trim().to_string())
            .and_then(|i| if i.is_empty() { None } else { Some(i) });

        let query_type = self.search_type.and_then(|i| match i {
            QueryType::Other => None,
            _ => Some(i),
        });

        QueryStruct {
            query: search_query,
            search_type: query_type,
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
