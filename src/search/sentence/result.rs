use diesel::sql_types::Text;

use crate::japanese::SentencePart;

#[derive(Debug, PartialEq, Clone, QueryableByName)]
pub struct Sentence {
    #[sql_type = "Text"]
    content: String,
    #[sql_type = "Text"]
    furigana: String,
    #[sql_type = "Text"]
    translation: String,
}

impl Sentence {
    fn furigana_pairs(&self) -> Vec<SentencePart> {
        vec![]
    }
}
