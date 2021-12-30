use actix_web::web::Json;
use resources::news;
use types::api::news::short::{Request, Response};

/// Get short news endpoint
pub async fn news(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    let after = payload.after;

    let entries = news::get()
        .last_entries(3)
        .filter(|i| i.creation_time > after)
        .map(|i| super::ne_from_resource(i, true))
        .collect::<Vec<_>>();

    Ok(Json(Response { entries }))
}
