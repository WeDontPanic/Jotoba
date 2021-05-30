use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use localization::TranslationDict;

use crate::{
    search_ep::parse_settings,
    templates, {BaseData, Site},
};

/// Homepage
pub async fn index(
    locale_dict: web::Data<Arc<TranslationDict>>,
    request: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let settings = parse_settings(&request);

    Ok(HttpResponse::Ok().body(render!(
        templates::base,
        BaseData::new(Site::Index, &locale_dict, settings)
    )))
}
