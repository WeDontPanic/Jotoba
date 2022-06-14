use actix_web::web::Json;
use error::api_error::RestError;
use types::api::kanji::ids_tree::{OutObject, Request};

use crate::kanji::ids_tree::builder::KanjiTreeBuilder;

pub mod builder;

/// Get short news endpoint
pub async fn decomp_graph(payload: Json<Request>) -> Result<Json<OutObject>, RestError> {
    let literal = payload.0.literal;

    let tree = KanjiTreeBuilder.build(literal).ok_or(RestError::NotFound)?;

    Ok(Json(tree))
}
