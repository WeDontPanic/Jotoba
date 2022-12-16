use actix_web::{web::Json, HttpResponse};
use error::api_error::RestError;
use types::{api::info::words::Request, jotoba::words::filter_languages};

/// TODO: make this privtate using a key or something
/// Handles a word info API request
pub async fn word_info(payload: Json<Request>) -> Result<HttpResponse, RestError> {
    let word_retr = resources::get().words();

    let mut words = payload
        .ids
        .iter()
        .filter_map(|i| word_retr.by_sequence(*i))
        .cloned()
        .collect::<Vec<_>>();

    filter_languages(words.iter_mut(), payload.language, payload.show_english);

    Ok(HttpResponse::Ok().body(bincode::serialize(&words).unwrap()))
}
