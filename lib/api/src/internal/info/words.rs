use std::collections::HashSet;

use actix_web::{web::Json, HttpResponse};
use error::api_error::RestError;
use types::{
    api::internal::info::words::{Request, Response, WordItem},
    jotoba::words::{part_of_speech::PosSimple, Word},
};

/// Handles a word info API request
pub async fn word_info(payload: Json<Request>) -> Result<HttpResponse, RestError> {
    let word_retr = resources::get().words();

    let items: Vec<_> = payload
        .ids
        .iter()
        .filter_map(|i| word_retr.by_sequence(*i))
        .cloned()
        .map(|mut word| {
            word.adjust_language(payload.lang_param());
            let pos = unique_pos(&word);
            WordItem {
                sentences: vec![],
                audio: word.audio_file_name(),
                word,
                pos,
            }
        })
        .collect();

    let response = Response { items };
    Ok(HttpResponse::Ok().body(bincode::serialize(&response).unwrap()))
}

fn unique_pos(word: &Word) -> Vec<PosSimple> {
    word.senses()
        .into_iter()
        .map(|i| &i.part_of_speech)
        .flatten()
        .map(|i| i.to_pos_simple())
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
