use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use config::Config;
use localization::TranslationDict;
use search::{
    query::{Query, UserSettings},
    sentence::{self, result::SentenceResult},
    word::{self, result::WordResult},
};
use types::jotoba::{search::QueryType, words::filter_languages};

use crate::{
    search_ep::redirect_home,
    templates, user_settings,
    web_error::{self, Error},
    BaseData, ResultData,
};

/// Endpoint to perform a search
pub async fn direct_ep(
    h_query: web::Path<(u8, String)>,
    locale_dict: web::Data<Arc<TranslationDict>>,
    config: web::Data<Config>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let settings = user_settings::parse(&request);

    let (stype, id) = h_query.into_inner();
    let query_type = QueryType::try_from(stype).map_err(|_| Error::BadRequest)?;

    let result_data = match query_type {
        QueryType::Words => find_direct_word(&id, &settings).await,
        QueryType::Names => find_direct_name(&id).await,
        QueryType::Sentences => find_direct_sentence(&id).await,
        QueryType::Kanji => return Ok(redirect_home()),
    };

    if let Err(err) = result_data {
        return match err {
            web_error::Error::NotFound => Err(err),
            _ => Ok(redirect_home()),
        };
    }

    let query = Query::default();
    let base_data = BaseData::new(&locale_dict, settings, &config.asset_hash, &config)
        .with_search_result(&query, result_data.unwrap(), None);

    Ok(HttpResponse::Ok().body(render!(templates::base, base_data).render()))
}

/// Find direct word
pub async fn find_direct_word(id: &str, settings: &UserSettings) -> Result<ResultData, Error> {
    let sequence_id: u32 = id.parse().map_err(|_| Error::NotFound)?;

    let res_name = resources::get()
        .words()
        .by_sequence(sequence_id)
        .ok_or(web_error::Error::NotFound)?
        .clone();

    let mut restults = vec![res_name];

    filter_languages(restults.iter_mut(), settings.user_lang, true);

    let word = restults.remove(0);

    Ok(ResultData::Word(WordResult {
        items: vec![word::result::Item::Word(word)],
        count: 1,
        contains_kanji: false,
        inflection_info: None,
        sentence_parts: None,
        sentence_index: 0,
        searched_query: String::new(),
    }))
}

/// Find direct name
pub async fn find_direct_name(id: &str) -> Result<ResultData, Error> {
    let sequence_id: u32 = id.parse().map_err(|_| Error::NotFound)?;

    let res_word = resources::get()
        .names()
        .by_sequence(sequence_id)
        .ok_or(web_error::Error::NotFound)?;

    Ok(ResultData::Name(vec![res_word]))
}

/// Find direct sentence
pub async fn find_direct_sentence(id: &str) -> Result<ResultData, Error> {
    let sequence_id: u32 = id.parse().map_err(|_| Error::NotFound)?;

    let res_sentence = resources::get()
        .sentences()
        .by_id(sequence_id)
        .ok_or(web_error::Error::NotFound)?
        .clone();

    let res_sentence = sentence::result::Sentence::from_m_sentence(
        res_sentence,
        types::jotoba::languages::Language::English,
        true,
    )
    .unwrap();

    Ok(ResultData::Sentence(SentenceResult {
        items: vec![sentence::result::Item {
            sentence: res_sentence,
        }],
        len: 1,
        hidden: false,
    }))
}
