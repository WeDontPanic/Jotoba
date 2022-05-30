use serde::Serialize;

use crate::{
    api::{app::search::responses::kanji::Kanji, app::search::responses::words::Word},
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
    transitivity_pair: Option<TransitivityPair>,
}

#[derive(Serialize)]
#[serde(tag = "t", content = "w")]
pub enum TransitivityPair {
    Transitive(u32),
    Intransitive(u32),
}

impl Details {
    #[inline]
    pub fn new(
        word: Word,
        kanji: Vec<Kanji>,
        conjugations: Option<Inflections>,
        collocations: Vec<Word>,
        has_sentence: bool,
        transitivity_pair: Option<TransitivityPair>,
    ) -> Self {
        Self {
            word,
            kanji,
            conjugations,
            collocations,
            has_sentence,
            transitivity_pair,
        }
    }
}
