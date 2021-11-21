use resources::parse::jmdict::{
    dialect::Dialect, field::Field, languages::Language, misc::Misc, part_of_speech::PartOfSpeech,
};

use search::word::result::{Item, WordResult};
use serde::Serialize;

use crate::search::kanji::response::Kanji;

/// The API response struct for a word search
#[derive(Serialize)]
pub struct Response {
    kanji: Vec<Kanji>,
    words: Vec<Word>,
}

/// Represents a single Word result with 1 (main) Japanese reading and n glosses
#[derive(Serialize)]
pub struct Word {
    reading: Reading,
    common: bool,
    senses: Vec<Sense>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt_readings: Option<Vec<Reading>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<String>,
}

#[derive(Serialize)]
pub struct Reading {
    kana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    furigana: Option<String>,
}

#[derive(Serialize)]
pub struct Sense {
    glosses: Vec<String>,
    pos: Vec<PartOfSpeech>,
    language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    dialect: Option<Dialect>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field: Option<Field>,
    #[serde(skip_serializing_if = "Option::is_none")]
    information: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    antonym: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    misc: Option<Misc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xref: Option<String>,
}

impl From<&resources::models::words::Sense> for Sense {
    fn from(sense: &resources::models::words::Sense) -> Self {
        let pos = sense.part_of_speech.clone();

        let glosses = sense
            .glosses
            .iter()
            .map(|i| i.gloss.clone())
            .collect::<Vec<_>>();

        Self {
            glosses,
            pos,
            language: sense.language,
            dialect: sense.dialect,
            field: sense.field,
            information: sense.information.as_ref().map(|i| i.clone()),
            antonym: sense.antonym.as_ref().map(|i| i.clone()),
            misc: sense.misc,
            xref: sense.xref.as_ref().map(|i| i.clone()),
        }
    }
}

impl From<&resources::models::words::Word> for Word {
    #[inline]
    fn from(word: &resources::models::words::Word) -> Self {
        let kanji = word.reading.kanji.as_ref().map(|i| i.reading.clone());
        let kana = word.reading.kana.clone().reading;
        let furigana = word.furigana.clone();

        let senses = word.senses.iter().map(|i| Sense::from(i)).collect();

        Self {
            common: word.is_common(),
            reading: Reading {
                kanji,
                kana,
                furigana,
            },
            senses,
            alt_readings: None,
            audio: word
                .audio_file("mp3")
                .as_ref()
                .map(|i| format!("/audio/{}", i)),
        }
    }
}

impl From<WordResult> for Response {
    #[inline]
    fn from(wres: WordResult) -> Self {
        let kanji = convert_kanji(&wres);
        let words = convert_words(&wres);

        Self { kanji, words }
    }
}

#[inline]
fn convert_kanji(wres: &WordResult) -> Vec<Kanji> {
    wres.items
        .iter()
        .filter_map(|i| match i {
            Item::Kanji(k) => Some(k.into()),
            _ => None,
        })
        .collect()
}

#[inline]
fn convert_words(wres: &WordResult) -> Vec<Word> {
    wres.items
        .iter()
        .filter_map(|i| match i {
            Item::Word(w) => Some(w.into()),
            _ => None,
        })
        .collect()
}
