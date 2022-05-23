use serde::Serialize;

use crate::{
    api::{app::search::responses::words::Word, search::kanji::Kanji},
    jotoba::words::inflection::Inflections,
};

#[derive(Serialize)]
pub struct Details {
    word: Word,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    kanji: Vec<Kanji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conjugations: Option<Inflections>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    collocations: Vec<Word>,
    has_sentence: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    transitiviti_pair: Option<TransitivityPair>,
}

#[derive(Serialize)]
pub enum TransitivityPair {
    Transitive(Word),
    Intransitive(Word),
}
