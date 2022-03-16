use crate::app::new_page;

use super::{convert_payload, Result};
use actix_web::web::{self, Json};
use error::api_error::RestError;
use types::{
    api::app::{query::SearchPayload, responses::names},
    jotoba::pagination::page::Page,
};

/// API response type
pub type Resp = Page<names::Response>;

/// Do an app name search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;

    let result = web::block(move || search::name::search(&query)).await??;
    let res = names::Response::new(result.items.into_iter().cloned().collect());
    let len = result.total_count as u32;
    let page = new_page(&payload, res, len, payload.settings.page_size);

    Ok(Json(page))
}
