use actix_web::web::{self, Json};
use search::SearchExecutor;
use types::{api::search::name::Response, jotoba::search::SearchTarget};

use super::{Result, SearchRequest};

/// Do a name search via API
pub async fn name_search(payload: Json<SearchRequest>) -> Result<Json<Response>> {
    let query = super::parse_query(payload, SearchTarget::Kanji)?;
    let result = web::block(move || {
        let search = search::name::Search::new(&query);
        SearchExecutor::new(search).run()
    })
    .await?;
    Ok(Json(result.items.into()))
}
