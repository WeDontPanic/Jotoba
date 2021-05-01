use diesel::sql_types::Text;

use crate::japanese::{self, SentencePart};

#[derive(Debug, PartialEq, Clone, QueryableByName)]
pub struct Sentence {
    #[sql_type = "Text"]
    pub content: String,
    #[sql_type = "Text"]
    furigana: String,
    #[sql_type = "Text"]
    pub translation: String,
}

impl Sentence {
    pub fn furigana_pairs(&self) -> Vec<SentencePart> {
        japanese::format_pairs(japanese::furigana_from_str(&self.furigana))
    }
}
