use actix_web::{web, HttpRequest, HttpResponse};

use crate::{templates, DbPool};

/// Homepage
pub async fn index(
    pool: web::Data<DbPool>,
    request: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(render!(templates::base, None, None)))
}
