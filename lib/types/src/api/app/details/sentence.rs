use serde::{Deserialize, Serialize};

use crate::api::app::search::responses::{kanji::Kanji, sentences::Sentence, words::Word};

#[derive(Serialize, Deserialize)]
pub struct Details {
    sentence: Sentence,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    words: Vec<Word>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    kanji: Vec<Kanji>,
}

impl Details {
    pub fn new(sentence: Sentence, words: Vec<Word>, kanji: Vec<Kanji>) -> Self {
        Self {
            sentence,
            words,
            kanji,
        }
    }
}

