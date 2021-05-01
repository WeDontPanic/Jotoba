use itertools::Itertools;
use tokio_diesel::AsyncRunQueryDsl;

use crate::schema::{sentence, sentence_translation};
use crate::{error::Error, parse::jmdict::languages::Language, DbPool};

#[derive(Queryable, Clone, Debug, PartialEq, QueryableByName)]
#[table_name = "sentence"]
pub struct Sentence {
    pub id: i32,
    pub content: String,
    pub furigana: String,
}

#[derive(Insertable, Clone, PartialEq)]
#[table_name = "sentence"]
pub struct NewSentence {
    pub content: String,
    pub furigana: String,
}

#[derive(Queryable, Clone, Debug, PartialEq)]
pub struct Translation {
    pub id: i32,
    pub sentence_id: i32,
    pub language: Language,
    pub content: String,
}

#[derive(Insertable, Clone, PartialEq)]
#[table_name = "sentence_translation"]
pub struct NewTranslation {
    pub sentence_id: i32,
    pub language: Language,
    pub content: String,
}

/// Inserts a new sentence into the DB
pub async fn insert_sentence(
    db: &DbPool,
    text: String,
    furigana: String,
    translations: Vec<(String, Language)>,
) -> Result<(), Error> {
    let new_sentence = NewSentence {
        content: text,
        furigana,
    };

    let sid: i32 = diesel::insert_into(sentence::table)
        .values(new_sentence)
        .returning(sentence::id)
        .get_result_async(db)
        .await?;

    let translations = translations
        .into_iter()
        .map(|i| NewTranslation {
            content: i.0,
            language: i.1,
            sentence_id: sid,
        })
        .collect_vec();

    diesel::insert_into(sentence_translation::table)
        .values(translations)
        .execute_async(db)
        .await?;

    Ok(())
}

/// Clear all sense entries
pub async fn clear(db: &DbPool) -> Result<(), Error> {
    diesel::delete(sentence_translation::table)
        .execute_async(db)
        .await?;
    diesel::delete(sentence::table).execute_async(db).await?;
    Ok(())
}
