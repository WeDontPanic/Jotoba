use actix_web::web::Json;
use error::api_error;
use resources::news;
use serde::{Deserialize, Serialize};

use super::NewsEntry;

#[derive(Deserialize)]
pub struct Request {
    pub id: u32,
}

#[derive(Serialize)]
pub struct Response {
    pub entry: NewsEntry,
}

/// Get detailed news endpoint
pub async fn news(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    let id = payload.id;

    let entry = news::get()
        .by_id(id)
        .map(|i| NewsEntry::from_resource(i, false))
        .ok_or_else(|| api_error::RestError::NotFound)?;

    Ok(Json(Response { entry }))
}
