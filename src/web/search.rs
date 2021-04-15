use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{templates, DbPool};

#[derive(Deserialize)]
pub struct QueryStruct {
    #[serde(rename = "search")]
    query: Option<String>,
}

/// Endpoint to perform a search
pub async fn search(
    pool: web::Data<DbPool>,
    query_data: web::Query<QueryStruct>,
) -> Result<HttpResponse, actix_web::Error> {
    let search_query = query_data
        .query
        .clone()
        .map(|i| i.trim().to_string())
        .and_then(|i| if i.is_empty() { None } else { Some(i) });

    let search_query = match search_query {
        Some(s) => s,
        None => {
            return Ok(HttpResponse::MovedPermanently()
                .header("Location", "/")
                .finish())
        }
    };

    let result = crate::search::everything::search(&pool, &search_query)
        .await
        .unwrap();

    Ok(HttpResponse::Ok().body(render!(
        templates::base,
        Some(search_query.clone()),
        Some(result)
    )))
}
