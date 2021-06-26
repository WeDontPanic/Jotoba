use models::kanji::KanjiResult;
use parse::jmdict::{
    dialect::Dialect, field::Field, languages::Language, misc::Misc, part_of_speech::PartOfSpeech,
};
use search::word::result::{self, Item, WordResult};
use serde::Serialize;

/// The API response struct for a word search
#[derive(Serialize, Default)]
pub struct Response {
    kanji: Vec<Kanji>,
    words: Vec<Word>,
}

/// Represents a single Kanji used to write one of the words occurring in word results
#[derive(Serialize, Default)]
pub struct Kanji {
    literal: String,
    meanings: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    grade: Option<i32>,
    stroke_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    jlpt: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    onyomi: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kunyomi: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chinese: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    korean_r: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    korean_h: Option<Vec<String>>,
}

impl From<&KanjiResult> for Kanji {
    fn from(kanji: &KanjiResult) -> Self {
        Self {
            literal: kanji.kanji.literal.clone(),
            meanings: kanji.meanings.clone(),
            grade: kanji.kanji.grade,
            stroke_count: kanji.kanji.stroke_count,
            frequency: kanji.kanji.frequency,
            jlpt: kanji.kanji.jlpt,
            variant: kanji.kanji.variant.clone(),
            onyomi: kanji.kanji.onyomi.clone(),
            kunyomi: kanji.kanji.kunyomi.clone(),
            chinese: kanji.kanji.chinese.clone(),
            korean_r: kanji.kanji.korean_r.clone(),
            korean_h: kanji.kanji.korean_h.clone(),
        }
    }
}

/// Represents a single Word result with 1 (main) Japanese reading and n glosses
#[derive(Serialize)]
pub struct Word {
    reading: Reading,
    common: bool,
    senses: Vec<Sense>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alt_readings: Option<Vec<Reading>>,
}

#[derive(Serialize, Default)]
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

impl From<&result::Sense> for Sense {
    fn from(sense: &result::Sense) -> Self {
        let mut pos = sense
            .glosses
            .iter()
            .map(|i| i.part_of_speech.clone())
            .flatten()
            .collect::<Vec<_>>();

        pos.sort();
        pos.dedup();

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

impl From<&result::Word> for Word {
    fn from(word: &result::Word) -> Self {
        let kanji = word.reading.kanji.as_ref().map(|i| i.reading.clone());
        let kana = word.reading.kana.clone().unwrap().reading;
        let furigana = word.reading.kanji.as_ref().and_then(|i| i.furigana.clone());

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
        }
    }
}

impl From<WordResult> for Response {
    fn from(wres: WordResult) -> Self {
        let kanji = convert_kanji(&wres);
        let words = convert_words(&wres);

        Self { kanji, words }
    }
}

fn convert_kanji(wres: &WordResult) -> Vec<Kanji> {
    wres.items
        .iter()
        .filter_map(|i| match i {
            Item::Kanji(k) => Some(k.into()),
            _ => None,
        })
        .collect()
}

fn convert_words(wres: &WordResult) -> Vec<Word> {
    wres.items
        .iter()
        .filter_map(|i| match i {
            Item::Word(w) => Some(w.into()),
            _ => None,
        })
        .collect()
}
