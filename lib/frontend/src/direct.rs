use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use config::Config;
use localization::TranslationDict;
use search::{
    query::Query,
    word::result::{Item, WordResult},
};
use types::jotoba::search::QueryType;

use crate::{templates, user_settings, web_error, BaseData, ResultData};

/// Endpoint to perform a search
pub async fn direct_ep(
    h_query: web::Path<(u8, String)>,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, web_error::Error> {
    let settings = user_settings::parse(&request);

    let (stype, id) = h_query.into_inner();
    let query_type = QueryType::try_from(stype).map_err(|_| web_error::Error::BadRequest)?;

    let result_data = match query_type {
        QueryType::Kanji => todo!(),
        QueryType::Sentences => todo!(),
        QueryType::Names => todo!(),
        QueryType::Words => find_direct_word(&id).await,
    }?;

    let query = Query::default();
    let base_data = BaseData::new(&locale_dict, settings, &config.asset_hash, &config)
        .with_search_result(&query, result_data, None);

    Ok(HttpResponse::Ok().body(render!(templates::base, base_data).render()))
}

/// Find direct word
pub async fn find_direct_word(id: &str) -> Result<ResultData, web_error::Error> {
    let sequence_id: u32 = id.parse().map_err(|_| web_error::Error::BadRequest)?;

    let res_word = resources::get()
        .words()
        .by_sequence(sequence_id)
        .ok_or(web_error::Error::NotFound)?
        .clone();

    Ok(ResultData::Word(WordResult {
        items: vec![Item::Word(res_word)],
        count: 1,
        contains_kanji: false,
        inflection_info: None,
        sentence_parts: None,
        sentence_index: 0,
        searched_query: String::new(),
    }))
}
