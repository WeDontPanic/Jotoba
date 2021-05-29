use actix_web::HttpResponse;

use crate::{
    templates, {BaseData, Site},
};

/// About page
pub async fn about() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(render!(templates::base, BaseData::new(Site::About))))
}
