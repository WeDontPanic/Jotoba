pub mod completions;
pub mod details;
pub mod img;
pub mod kanji;
pub mod news;
pub mod radical;
pub mod search;

use std::path::Path;

use config::Config;
use error::api_error::RestError;
use types::{
    api::app::search::responses::words,
    jotoba::{self, language::Language},
};

pub type Result<T> = std::result::Result<T, RestError>;

pub(crate) fn conv_word(word: jotoba::words::Word, lang: Language, config: &Config) -> words::Word {
    let is_common = word.is_common();
    let accents = word.get_pitches();

    let audio = word.audio_file_name().and_then(|name| {
        let audio_p = Path::new("mp3").join(name);
        let local_path = Path::new(config.server.get_audio_files()).join(&audio_p);
        if local_path.exists() {
            let url = Path::new("/audio/")
                .join(&audio_p)
                .to_str()
                .unwrap()
                .to_string();
            Some(url)
        } else {
            None
        }
    });

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
        accents,
        jlpt_lvl: word.jlpt_lvl.map(|i| i.get()),
        furigana: word.furigana,
        transive_version: word.transive_version.map(|i| i.get()),
        intransive_version: word.intransive_version.map(|i| i.get()),
        sentences_available: word.sentences_available,
        audio,
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
        .translation_for(language)
        .or_else(|| sentence.translation_for(Language::English))?;

    Some((sentence.furigana.clone(), translation.to_string()))
}
