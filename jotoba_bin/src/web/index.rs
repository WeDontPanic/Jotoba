use actix_web::HttpResponse;

use crate::{
    templates,
    web::{BaseData, Site},
};

/// Homepage
pub async fn index() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body(render!(templates::base, BaseData::new(Site::Index))))
}
