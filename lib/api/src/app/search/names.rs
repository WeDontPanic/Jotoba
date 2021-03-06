use super::new_page;

use super::convert_payload;
use crate::app::Result;
use actix_web::web::{self, Json};
use error::api_error::RestError;
use search::SearchExecutor;
use types::{
    api::app::search::{
        query::SearchPayload,
        responses::{names, Response},
    },
    jotoba::search::SearchTarget,
};

/// API response type
pub type Resp = Response<names::Response>;

/// Do an app name search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;

    let query_c = query.clone();
    let result = web::block(move || {
        let search = search::name::Search::new(&query_c);
        SearchExecutor::new(search).run()
    })
    .await?;
    let res = names::Response::new(result.items.into_iter().cloned().collect());
    let len = result.total as u32;
    let page = new_page(&payload, res, len, payload.settings.page_size);
    let res = super::new_response(page, SearchTarget::Names, &query);
    Ok(Json(res))
}
