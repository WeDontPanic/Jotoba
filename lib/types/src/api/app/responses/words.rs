use serde::Serialize;

use crate::jotoba::words::{inflection::Inflection, Word};

/// A word search response
#[derive(Clone, Serialize)]
pub struct Response {
    /// All word results for the current search
    words: Vec<Word>,

    /// Kanji used in words
    kanji: Vec<super::kanji::Kanji>,

    /// Inflection information of the current word
    infl_info: Option<InflectionInfo>,

    /// Sentence reader data
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

#[derive(Clone, Serialize)]
pub struct InflectionInfo {
    inflections: Vec<Inflection>,
    /// The "uninflected" version
    lexeme: String,
}

impl InflectionInfo {
    /// Create a new InflectionInfo
    #[inline]
    pub fn new(inflection: Vec<Inflection>, lexeme: String) -> Self {
        Self {
            inflections: inflection,
            lexeme,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Sentence {
    /// Currently selected part
    curr_index: usize,
    /// All Parts of the sentence
    parts: Vec<SentencePart>,
}

impl Sentence {
    pub fn new(curr_index: usize, parts: Vec<SentencePart>) -> Self {
        Self { curr_index, parts }
    }
}

#[derive(Clone, Serialize)]
pub struct SentencePart {
    /// Original inflected word
    inflected: String,
    /// Furigana of the inflected word. None if can't be
    /// calculated or word is completetly in kana
    furigana: Option<String>,
    /// Position of the sentence_part in the sentence
    position: usize,
    /// Part of Speech
    word_class: Option<&'static str>,
}

impl SentencePart {
    #[inline]
    pub fn new(
        furigana: Option<String>,
        position: usize,
        inflected: String,
        word_class: Option<&'static str>,
    ) -> Self {
        Self {
            furigana,
            position,
            inflected,
            word_class,
        }
    }
}
