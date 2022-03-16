use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Sentence {
    /// Currently selected part
    curr_index: usize,
    /// All Parts of the sentence
    parts: Vec<SentencePart>,
}

impl Sentence {
    #[inline]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    furigana: Option<String>,
    /// Position of the sentence_part in the sentence
    position: usize,
    /// Part of Speech
    #[serde(skip_serializing_if = "Option::is_none")]
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
