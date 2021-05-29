use diesel::sql_types::{Integer, Text};

use japanese::{furigana, furigana::SentencePartRef};
use parse::jmdict::languages::Language;

#[derive(Debug, PartialEq, Clone, QueryableByName)]
pub struct Sentence {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub content: String,
    #[sql_type = "Text"]
    pub furigana: String,
    #[sql_type = "Text"]
    pub translation: String,
    #[sql_type = "Integer"]
    pub language: Language,
    #[sql_type = "Text"]
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
