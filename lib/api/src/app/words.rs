use crate::app::new_page;

use super::{convert_payload, Result};
use actix_web::web::{self, Json};
use error::api_error::RestError;
use types::{
    api::app::{
        query::SearchPayload,
        responses::words::{self, Sentence},
    },
    jotoba::{self, languages::Language, pagination::page::Page},
};

/// API response type
pub type Resp = Page<words::Response>;

/// Do an app word search via API
pub async fn search(payload: Json<SearchPayload>) -> Result<Json<Resp>> {
    let query = convert_payload(&payload)
        .parse()
        .ok_or(RestError::BadRequest)?;
    let user_lang = query.settings.user_lang;

    let result = web::block(move || search::word::search(&query)).await??;

    let (words, kanji) = result.get_items();
    let kanji = conv_kanji(&kanji);
    let words = words
        .into_iter()
        .map(|i| conv_word(i.clone(), user_lang))
        .collect();

    let sentence = result
        .sentence_parts
        .map(|i| conv_sentence(i, result.sentence_index));
    let infl_info = result.inflection_info.map(|i| conv_infl_info(i));

    let original_query = result.searched_query;

    let res = words::Response::new(words, kanji, infl_info, sentence, original_query);
    let len = result.count as u32;

    let page = new_page(&payload, res, len, payload.settings.page_size);
    Ok(Json(page))
}

fn conv_kanji(
    kanji: &[&types::jotoba::kanji::Kanji],
) -> Vec<types::api::app::responses::kanji::Kanji> {
    kanji.iter().map(|i| (*i).clone().into()).collect()
}

fn conv_sentence(sentence: sentence_reader::Sentence, index: usize) -> Sentence {
    let parts = sentence
        .into_parts()
        .into_iter()
        .map(|i| i.into())
        .collect();
    Sentence::new(index, parts)
}

fn conv_infl_info(infl_info: search::word::result::InflectionInformation) -> words::InflectionInfo {
    words::InflectionInfo::new(infl_info.inflections, infl_info.lexeme)
}

pub fn conv_word(word: jotoba::words::Word, lang: Language) -> words::Word {
    let is_common = word.is_common();

    let reading = word
        .furigana
        .as_ref()
        .map(|i| i.clone())
        .unwrap_or(word.get_reading().reading.clone());

    let alt_readings = word
        .reading
        .alternative
        .into_iter()
        .map(|i| i.reading)
        .collect();

    let senses = word
        .senses
        .into_iter()
        .map(|i| conv_ex_sentence(i, lang))
        .collect::<Vec<_>>();

    words::Word {
        sequence: word.sequence,
        is_common,
        reading,
        alt_readings,
        senses,
        accents: word.accents,
        furigana: word.furigana,
        jlpt_lvl: word.jlpt_lvl,
        transive_verion: word.transive_verion,
        intransive_verion: word.intransive_verion,
        sentences_available: word.sentences_available,
    }
}

#[inline]
pub fn conv_ex_sentence(sense: jotoba::words::sense::Sense, lang: Language) -> words::Sense {
    let glosses = sense
        .glosses
        .into_iter()
        .map(|i| i.gloss)
        .collect::<Vec<_>>();

    let example_sentence = sense
        .example_sentence
        .and_then(|i| get_example_sentence(i, lang));

    words::Sense {
        misc: sense.misc,
        field: sense.field,
        dialect: sense.dialect,
        glosses,
        xref: sense.xref,
        antonym: sense.antonym,
        information: sense.information,
        part_of_speech: sense.part_of_speech,
        language: sense.language,
        example_sentence,
        gairaigo: sense.gairaigo,
    }
}

fn get_example_sentence(id: u32, language: Language) -> Option<(String, String)> {
    let sentence = resources::get().sentences().by_id(id)?;

    let translation = sentence
        .get_translations(language)
        .or_else(|| sentence.get_translations(Language::English))?;

    Some((sentence.furigana.clone(), translation.to_string()))
}
