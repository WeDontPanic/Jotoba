use diesel::sql_types::{Integer, Text};

use crate::{
    error::Error,
    japanese::{self, SentencePart},
    models::sentence::SentenceVocabulary,
    DbPool,
};

use super::sentencesearch::SentenceSearch;

#[derive(Debug, PartialEq, Clone, QueryableByName)]
pub struct Sentence {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub content: String,
    #[sql_type = "Text"]
    furigana: String,
    #[sql_type = "Text"]
    pub translation: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub sentence: Sentence,
    pub vocabularies: Vec<(String, SentenceVocabulary)>,
}

impl Sentence {
    pub fn furigana_pairs(&self) -> Vec<SentencePart> {
        japanese::format_pairs(japanese::furigana_from_str(&self.furigana))
    }

    pub async fn into_item(self, db: &DbPool) -> Result<Item, Error> {
        let vocs = SentenceSearch::get_vocabularies(db, self.id).await?;
        println!("vocs: {:#?}", vocs);
        println!("sparts: {:#?}", self.furigana_pairs());

        Ok(Item {
            vocabularies: vocs,
            sentence: self,
        })
    }
}
