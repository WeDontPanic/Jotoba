use actix_web::web::Json;
use error::api_error;
use resources::news;
use types::api::news::long::{Request, Response};

/// Get detailed news endpoint
pub async fn news(payload: Json<Request>) -> Result<Json<Response>, actix_web::Error> {
    let id = payload.id;

    let entry = news::get()
        .by_id(id)
        .map(|i| super::ne_from_resource(i, false))
        .ok_or(api_error::RestError::NotFound)?;

    Ok(Json(Response { entry }))
}
