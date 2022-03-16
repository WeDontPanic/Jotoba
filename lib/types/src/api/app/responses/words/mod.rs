mod inflection;
mod sentence;
mod word;

pub use inflection::*;
pub use sentence::*;
pub use word::*;

use serde::Serialize;

/// A word search response
#[derive(Clone, Serialize)]
pub struct Response {
    /// All word results for the current search
    words: Vec<Word>,

    /// Kanji used in words
    #[serde(skip_serializing_if = "Vec::is_empty")]
    kanji: Vec<super::kanji::Kanji>,

    /// Inflection information of the current word
    #[serde(skip_serializing_if = "Option::is_none")]
    infl_info: Option<InflectionInfo>,

    /// Sentence reader data
    #[serde(skip_serializing_if = "Option::is_none")]
    sentence: Option<Sentence>,

    /// Query that has actually been used for search
    original_query: String,
}

impl Response {
    /// Create a new Response
    pub fn new(
        words: Vec<Word>,
        kanji: Vec<super::kanji::Kanji>,
        infl_info: Option<InflectionInfo>,
        sentence: Option<Sentence>,
        original_query: String,
    ) -> Self {
        Self {
            words,
            kanji,
            infl_info,
            sentence,
            original_query,
        }
    }
}
