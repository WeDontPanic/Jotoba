use actix_web::web::Json;
use resources::news;
use serde::{Deserialize, Serialize};

use super::NewsEntry;

#[derive(Deserialize)]
pub struct Request {
    pub after: u64,
}

#[derive(Serialize)]
pub struct Response {
    pub entries: Vec<NewsEntry>,
}

/// Get short news endpoint
pub async fn news(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    let after = payload.after;

    let entries = news::get()
        .last_entries(3)
        .filter(|i| i.creation_time > after)
        .map(|i| NewsEntry::from_resource(i, true))
        .collect::<Vec<_>>();

    Ok(Json(Response { entries }))
}
