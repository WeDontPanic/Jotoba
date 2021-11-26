use resources::parse::jmdict::languages::Language;
use search::sentence;
use sentence::result;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    sentences: Vec<Sentence>,
}

#[derive(Serialize)]
pub struct Sentence {
    pub content: String,
    pub furigana: String,
    pub translation: String,
    pub language: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eng: Option<String>,
}

impl From<result::Sentence> for Sentence {
    #[inline]
    fn from(sentence: result::Sentence) -> Self {
        Self {
            eng: sentence.get_english().map(|i| i.to_owned()),
            content: sentence.content,
            furigana: sentence.furigana,
            translation: sentence.translation,
            language: sentence.language,
        }
    }
}

impl From<Vec<result::Sentence>> for Response {
    #[inline]
    fn from(sentences: Vec<result::Sentence>) -> Self {
        let sentences = sentences.into_iter().map(Sentence::from).collect();
        Self { sentences }
    }
}
