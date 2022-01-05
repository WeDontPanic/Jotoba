use std::sync::Arc;

//use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use config::Config;
use localization::TranslationDict;

use crate::{
    templates, user_settings, {BaseData, Site},
};

/// Homepage
pub async fn index(
    locale_dict: web::Data<Arc<TranslationDict>>,
    request: HttpRequest,
    config: web::Data<Config>,
) -> Result<HttpResponse, actix_web::Error> {
    let settings = user_settings::parse(&request);

    //session::init(&session, &settings);

    Ok(HttpResponse::Ok().body(
        render!(
            templates::base_index,
            BaseData::new(&locale_dict, settings, &config.asset_hash, &config)
                .with_site(Site::Index)
        )
        .render(),
    ))
}
