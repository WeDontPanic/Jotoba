use japanese::{furigana, furigana::SentencePartRef};
use parse::jmdict::languages::Language;

#[derive(Debug, PartialEq, Clone)]
pub struct Sentence {
    pub id: i32,
    pub content: String,
    pub furigana: String,
    pub translation: String,
    pub language: Language,
    pub eng: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub sentence: Sentence,
}

impl Sentence {
    pub fn furigana_pairs<'a>(&'a self) -> impl Iterator<Item = SentencePartRef<'a>> {
        furigana::from_str(&self.furigana)
    }

    pub fn get_english(&self) -> Option<&str> {
        if self.eng == "-" {
            None
        } else {
            Some(&self.eng)
        }
    }
}
