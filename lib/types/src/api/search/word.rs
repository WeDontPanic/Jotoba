use crate::{
    api::search::kanji::Kanji,
    jotoba::{
        languages::Language,
        words::{dialect::Dialect, field::Field, misc::Misc, part_of_speech::PartOfSpeech},
    },
};

use serde::{Deserialize, Serialize};

/// The API response struct for a word search
#[derive(Serialize, Deserialize)]
pub struct Response {
    kanji: Vec<Kanji>,
    words: Vec<Word>,
}

/// Represents a single Word result with 1 (main) Japanese reading and n glosses
#[derive(Serialize, Deserialize)]
pub struct Word {
    reading: Reading,
    common: bool,
    senses: Vec<Sense>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt_readings: Option<Vec<Reading>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pitch: Option<Vec<PitchItem>>,
}

#[derive(Serialize, Deserialize)]
pub struct PitchItem {
    part: String,
    high: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Reading {
    kana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    kanji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    furigana: Option<String>,
}

#[derive(Serialize, Deserialize)]
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

impl From<&crate::jotoba::words::sense::Sense> for Sense {
    fn from(sense: &crate::jotoba::words::sense::Sense) -> Self {
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
            information: sense.information.as_ref().cloned(),
            antonym: sense.antonym.as_ref().cloned(),
            misc: sense.misc,
            xref: sense.xref.as_ref().cloned(),
        }
    }
}

#[cfg(feature = "jotoba_intern")]
impl From<&crate::jotoba::words::Word> for Word {
    #[inline]
    fn from(word: &crate::jotoba::words::Word) -> Self {
        let kanji = word.reading.kanji.as_ref().map(|i| i.reading.clone());
        let kana = word.reading.kana.clone().reading;
        let furigana = word.furigana.clone();

        let senses = word.senses.iter().map(|i| Sense::from(i)).collect();

        let pitch = word.accents.as_ref().and_then(|accents| {
            japanese::accent::calc_pitch(&word.reading.kana.reading, accents[0] as i32)
                .map(|i| i.into_iter().map(|j| j.into()).collect::<Vec<PitchItem>>())
        });

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
                .audio_file("ogg")
                .as_ref()
                .map(|i| format!("/audio/{}", i)),
            pitch,
        }
    }
}

#[cfg(feature = "jotoba_intern")]
impl
    From<(
        Vec<&crate::jotoba::words::Word>,
        Vec<&crate::jotoba::kanji::Kanji>,
    )> for Response
{
    #[inline]
    fn from(
        wres: (
            Vec<&crate::jotoba::words::Word>,
            Vec<&crate::jotoba::kanji::Kanji>,
        ),
    ) -> Self {
        let kanji = convert_kanji(wres.1);
        let words = convert_words(wres.0);

        Self { kanji, words }
    }
}

#[cfg(feature = "jotoba_intern")]
#[inline]
fn convert_kanji(wres: Vec<&crate::jotoba::kanji::Kanji>) -> Vec<Kanji> {
    wres.into_iter().map(|i| i.into()).collect()
}

#[cfg(feature = "jotoba_intern")]
#[inline]
fn convert_words(wres: Vec<&crate::jotoba::words::Word>) -> Vec<Word> {
    wres.into_iter().map(|i| i.into()).collect()
}

#[cfg(feature = "jotoba_intern")]
impl From<(&str, bool)> for PitchItem {
    #[inline]
    fn from((part, high): (&str, bool)) -> Self {
        Self {
            part: part.to_owned(),
            high,
        }
    }
}
