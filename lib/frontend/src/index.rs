use std::sync::Arc;

use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use localization::TranslationDict;

use crate::{
    session, templates, user_settings, {BaseData, Site},
};

/// Homepage
pub async fn index(
    locale_dict: web::Data<Arc<TranslationDict>>,
    request: HttpRequest,
    session: Session,
) -> Result<HttpResponse, actix_web::Error> {
    let settings = user_settings::parse(&request);

    session::init(&session, &settings);

    Ok(HttpResponse::Ok().body(render!(
        templates::base,
        BaseData::new(Site::Index, &locale_dict, settings)
    )))
}
