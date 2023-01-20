use crate::app::{search::sentences::convert_sentence, Result};
use actix_web::web::{Data, Json};
use config::Config;
use engine::task::SearchTask;
use error::api_error::RestError;
use jp_utils::JapaneseExt;
use search::{engine::words::native::Engine, word::order::native::NativeOrder};
use sentence_reader::output::ParseResult;
use types::{
    api::app::{
        details::{query::DetailsPayload, sentence},
        search::responses::{kanji::Kanji, words::Word},
    },
    jotoba::{sentences::Sentence, words::filter_languages},
};

pub async fn details_ep(
    payload: Json<DetailsPayload>,
    config: Data<Config>,
) -> Result<Json<sentence::Details>> {
    Ok(Json(
        sentence_details(&payload, &config).ok_or(RestError::NotFound)?,
    ))
}

fn sentence_details(payload: &DetailsPayload, config: &Config) -> Option<sentence::Details> {
    let sentence = resources::get().sentences().by_id(payload.sequence)?;

    let kanji = get_kanji(sentence);

    let words = get_words(sentence, payload, config);

    let sentence =
        search::sentence::result::Sentence::from_m_sentence(sentence, payload.lang_param())?;

    let sentence = convert_sentence(sentence);
    Some(sentence::Details::new(sentence, words, kanji))
}

fn get_kanji(sentence: &Sentence) -> Vec<Kanji> {
    let kanji_iter = sentence.japanese.chars().filter(|i| i.is_kanji());

    let mut out: Vec<Kanji> = vec![];

    for k_lit in kanji_iter {
        if let Some(kanji) = resources::get().kanji().by_literal(k_lit) {
            out.push(kanji.to_owned().into());
        }
    }

    out
}

fn get_words(sentence: &Sentence, payload: &DetailsPayload, config: &Config) -> Vec<Word> {
    let parsed = sentence_reader::Parser::new(&sentence.japanese).parse();

    match parsed {
        ParseResult::Sentence(s) => s
            .iter()
            .map(|i| i.get_normalized())
            .filter_map(|i| find_word(&i, payload, config))
            .collect::<Vec<_>>(),
        ParseResult::InflectedWord(i) => find_word(&i.get_normalized(), payload, config)
            .map(|i| vec![i])
            .unwrap_or_default(),
        ParseResult::None => vec![],
    }
}

fn find_word(w: &str, payload: &DetailsPayload, config: &Config) -> Option<Word> {
    let mut task = SearchTask::<Engine>::new(w)
        .with_limit(4)
        .with_threshold(0.8)
        .with_custom_order(NativeOrder::new(w.to_string()));

    let res = task.find();
    if res.len() == 0 {
        return None;
    }

    let mut word = vec![res.into_inner().remove(0).item.clone()];
    filter_languages(word.iter_mut(), payload.lang_param());
    let word = super::super::conv_word(word.remove(0), payload.language, config);

    Some(word)
}
