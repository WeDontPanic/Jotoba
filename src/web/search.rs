use actix_web::{web, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::{templates, DbPool};

#[derive(Deserialize)]
pub struct QueryStruct {
    #[serde(rename = "search")]
    query: Option<String>,
}

use tokio_diesel::*;

/// Endpoint to perform a search
pub async fn search(
    pool: web::Data<DbPool>,
    request: HttpRequest,
    query_data: web::Query<QueryStruct>,
) -> Result<HttpResponse, actix_web::Error> {
    let search_query = match query_data.query.clone() {
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
