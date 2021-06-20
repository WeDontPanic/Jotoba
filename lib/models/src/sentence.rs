use deadpool_postgres::Pool;
use diesel::RunQueryDsl;
use itertools::Itertools;
use tokio_postgres::{types::ToSql, Row};

use crate::{
    queryable::{prepared_query, prepared_query_one, Deletable, Insertable, SQL},
    schema::{sentence, sentence_translation, sentence_vocabulary},
    DbConnection,
};
use error::Error;
use japanese;
use parse::jmdict::languages::Language;

use super::dict;

#[derive(Queryable, Clone, Debug, PartialEq, QueryableByName)]
#[table_name = "sentence"]
pub struct Sentence {
    pub id: i32,
    pub content: String,
    pub kana: String,
    pub furigana: String,
}

impl SQL for Sentence {
    fn get_tablename() -> &'static str {
        "sentence"
    }
}

#[derive(Insertable, Clone, PartialEq)]
#[table_name = "sentence"]
pub struct NewSentence {
    pub content: String,
    pub kana: String,
    pub furigana: String,
}

impl SQL for NewSentence {
    fn get_tablename() -> &'static str {
        "sentence"
    }
}

impl Insertable<3> for NewSentence {
    fn column_names() -> [&'static str; 3] {
        ["content", "kana", "furigana"]
    }

    fn fields(&self) -> [&(dyn ToSql + Sync); 3] {
        [&self.content, &self.kana, &self.furigana]
    }
}

#[derive(Queryable, Clone, Debug, PartialEq)]
pub struct Translation {
    pub id: i32,
    pub sentence_id: i32,
    pub language: Language,
    pub content: String,
}

impl SQL for Translation {
    fn get_tablename() -> &'static str {
        "sentence_translation"
    }
}

#[derive(Insertable, Clone, PartialEq)]
#[table_name = "sentence_translation"]
pub struct NewTranslation {
    pub sentence_id: i32,
    pub language: Language,
    pub content: String,
}

impl SQL for NewTranslation {
    fn get_tablename() -> &'static str {
        "sentence_translation"
    }
}

impl Insertable<3> for NewTranslation {
    fn column_names() -> [&'static str; 3] {
        ["sentence_id", "language", "content"]
    }

    fn fields(&self) -> [&(dyn ToSql + Sync); 3] {
        [&self.sentence_id, &self.language, &self.content]
    }
}

#[derive(Queryable, Clone, PartialEq, Debug)]
pub struct SentenceVocabulary {
    pub id: i32,
    pub sentence_id: i32,
    pub dict_sequence: i32,
    pub start: i32,
}

impl SQL for SentenceVocabulary {
    fn get_tablename() -> &'static str {
        "sentence_vocabulary"
    }
}

#[derive(Insertable, Clone, PartialEq)]
#[table_name = "sentence_vocabulary"]
pub struct NewSentenceVocabulary {
    pub sentence_id: i32,
    pub dict_sequence: i32,
    pub start: i32,
}

impl SQL for NewSentenceVocabulary {
    fn get_tablename() -> &'static str {
        "sentence_vocabulary"
    }
}

impl Insertable<3> for NewSentenceVocabulary {
    fn column_names() -> [&'static str; 3] {
        ["sentence_id", "dict_sequence", "start"]
    }

    fn fields(&self) -> [&(dyn ToSql + Sync); 3] {
        [&self.sentence_id, &self.dict_sequence, &self.start]
    }
}

impl From<Row> for Sentence {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            content: row.get(1),
            kana: row.get(2),
            furigana: row.get(3),
        }
    }
}

/// Inserts a new sentence into the DB
pub async fn insert_sentence(
    db: &Pool,
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
async fn generate_dict_relations(db: &Pool, sentence_id: i32, text: String) -> Result<(), Error> {
    let lexemes = japanese::jp_parsing::JA_NL_PARSER
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

    NewSentenceVocabulary::insert(db, &readings).await?;

    Ok(())
}

/// Insert a new sentence into DB and returns the created sentence ID
async fn insert_new_sentence(
    db: &Pool,
    text: String,
    furigana: String,
    translations: Vec<(String, Language)>,
) -> Result<i32, Error> {
    let kana = japanese::furigana::from_str(&furigana)
        .map(|i| i.kana)
        .join("");

    let new_sentence = NewSentence {
        content: text,
        furigana,
        kana,
    };

    let sql = format!("{} RETURNING id", NewSentence::get_insert_query(1));
    let sid: i32 =
        prepared_query_one(db, &sql, &NewSentence::get_bind_data(&[new_sentence])).await?;

    let translations = translations
        .into_iter()
        .map(|i| NewTranslation {
            content: i.0,
            language: i.1,
            sentence_id: sid,
        })
        .collect_vec();

    NewTranslation::insert(db, &translations).await?;

    Ok(sid)
}

/// Clear all sentence entries
pub async fn clear(db: &Pool) -> Result<(), Error> {
    Translation::delete_all(db).await?;
    SentenceVocabulary::delete_all(db).await?;
    Sentence::delete_all(db).await?;
    Ok(())
}
