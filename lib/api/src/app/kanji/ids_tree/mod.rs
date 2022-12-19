pub mod builder;

use actix_web::web::Json;
use builder::KanjiTreeBuilder;
use error::api_error::RestError;
use types::api::app::kanji::ids_tree::{Request, Response};

/// Get a decomposition graph
pub async fn decomp_graph(payload: Json<Request>) -> Result<Json<Response>, RestError> {
    let tree = KanjiTreeBuilder::new(payload.full)
        .build(payload.literal)
        .ok_or(RestError::NotFound)?;

    let size_opposite = KanjiTreeBuilder::new(!payload.full)
        .build(payload.literal)
        .ok_or(RestError::NotFound)?;

    let has_big = tree != size_opposite;

    Ok(Json(Response::new(tree, has_big)))
}
