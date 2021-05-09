use itertools::Itertools;
use tokio_diesel::AsyncRunQueryDsl;

use crate::schema::{sentence, sentence_translation, sentence_vocabulary};
use crate::{error::Error, parse::jmdict::languages::Language, DbPool};

use super::dict;

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

#[derive(Queryable, Clone, PartialEq, Debug)]
pub struct SentenceVocabulary {
    pub id: i32,
    pub sentence_id: i32,
    pub dict_sequence: i32,
    pub start: i32,
}

#[derive(Insertable, Clone, PartialEq)]
#[table_name = "sentence_vocabulary"]
pub struct NewSentenceVocabulary {
    pub sentence_id: i32,
    pub dict_sequence: i32,
    pub start: i32,
}

/// Inserts a new sentence into the DB
pub async fn insert_sentence(
    db: &DbPool,
    text: String,
    furigana: String,
    translations: Vec<(String, Language)>,
) -> Result<(), Error> {
    let _sentence_id = insert_new_sentence(db, text.clone(), furigana, translations).await?;

    #[cfg(feature = "tokenizer")]
    generate_dict_relations(db, _sentence_id, text).await?;

    Ok(())
}

/// Generates the relations between sentences and its words from [`dict`]
#[cfg(feature = "tokenizer")]
async fn generate_dict_relations(db: &DbPool, sentence_id: i32, text: String) -> Result<(), Error> {
    let lexemes = crate::JA_NL_PARSER
        .parse(&text)
        .into_iter()
        .collect::<Vec<_>>();

    let readings = dict::find_by_reading(
        db,
        &lexemes
            .iter()
            .map(|i| (i.lexeme, i.start as i32, i.word_class.is_particle()))
            .collect::<Vec<_>>(),
    )
    .await?
    .into_iter()
    .map(|(sequence, start)| NewSentenceVocabulary {
        sentence_id,
        dict_sequence: sequence,
        start,
    })
    .collect::<Vec<_>>();

    diesel::insert_into(sentence_vocabulary::table)
        .values(&readings)
        .execute_async(db)
        .await?;

    Ok(())
}

/// Insert a new sentence into DB and returns the created sentence ID
async fn insert_new_sentence(
    db: &DbPool,
    text: String,
    furigana: String,
    translations: Vec<(String, Language)>,
) -> Result<i32, Error> {
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

    Ok(sid)
}

/// Clear all sentence entries
pub async fn clear(db: &DbPool) -> Result<(), Error> {
    diesel::delete(sentence_translation::table)
        .execute_async(db)
        .await?;
    diesel::delete(sentence_vocabulary::table)
        .execute_async(db)
        .await?;
    diesel::delete(sentence::table).execute_async(db).await?;
    Ok(())
}
