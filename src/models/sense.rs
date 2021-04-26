use super::super::schema::sense;
use crate::{
    error::Error,
    parse::jmdict::Entry,
    parse::jmdict::{
        dialect::Dialect,
        field::Field,
        gtype::GType,
        languages::Language,
        misc::Misc,
        part_of_speech::{PartOfSpeech, PosSimple},
    },
    DbPool,
};
use diesel::prelude::*;
use tokio_diesel::*;

#[derive(Queryable, Clone, Debug)]
pub struct Sense {
    pub id: i32,
    pub sequence: i32,
    pub language: Language,
    pub gloss_pos: i32,
    pub gloss: String,
    pub misc: Option<Misc>,
    pub part_of_speech: Option<Vec<PartOfSpeech>>,
    pub dialect: Option<Dialect>,
    pub xref: Option<String>,
    pub gtype: Option<GType>,
    pub field: Option<Field>,
    pub information: Option<String>,
    pub antonym: Option<String>,
    pub pos_simplified: Option<Vec<PosSimple>>,
}

impl PartialEq for Sense {
    fn eq(&self, other: &Sense) -> bool {
        self.id == other.id && self.sequence == other.sequence
    }
}

#[derive(Insertable, Clone, PartialEq)]
#[table_name = "sense"]
pub struct NewSense {
    pub sequence: i32,
    pub language: Language,
    pub gloss_pos: i32,
    pub gloss: String,
    pub misc: Option<Misc>,
    pub part_of_speech: Option<Vec<PartOfSpeech>>,
    pub dialect: Option<Dialect>,
    pub xref: Option<String>,
    pub gtype: Option<GType>,
    pub field: Option<Field>,
    pub information: Option<String>,
    pub antonym: Option<String>,
    pub pos_simplified: Option<Vec<PosSimple>>,
}

/// Get all Database-dict structures from an entry
pub fn new_from_entry(entry: &Entry) -> Vec<NewSense> {
    let mut gloss_pos = -1;
    entry
        .senses
        .iter()
        .map(|item| {
            gloss_pos += 1;
            item.glosses
                .iter()
                .enumerate()
                .map(|(_, gloss)| NewSense {
                    sequence: entry.sequence as i32,
                    xref: item.xref.clone(),
                    dialect: item.dialect,
                    part_of_speech: (!item.part_of_speech.is_empty())
                        .then(|| item.part_of_speech.clone()),
                    gloss_pos,
                    gloss: gloss.value.clone(),
                    gtype: gloss.g_type,
                    misc: item.misc,
                    language: item.lang,
                    field: item.field,
                    antonym: item.antonym.clone(),
                    information: item.information.clone(),
                    pos_simplified: (!item.part_of_speech.is_empty())
                        .then(|| pos_simplified(&item.part_of_speech)),
                })
                .collect::<Vec<NewSense>>()
        })
        .flatten()
        .collect()
}

pub fn pos_simplified(pos: &[PartOfSpeech]) -> Vec<PosSimple> {
    pos.iter().map(|i| (*i).into()).collect()
}

/// Returns Ok(true) if at least one sense exists in the Db
pub async fn exists(db: &DbPool) -> Result<bool, Error> {
    use crate::schema::sense::dsl::*;
    Ok(sense
        .select((id, sequence))
        .limit(1)
        .execute_async(db)
        .await?
        == 1)
}

/// Insert multiple dicts into the database
pub async fn insert_sense(db: &DbPool, senses: Vec<NewSense>) -> Result<(), Error> {
    use crate::schema::sense::dsl::*;

    diesel::insert_into(sense)
        .values(senses)
        .execute_async(db)
        .await?;

    Ok(())
}

/// Clear all sense entries
pub async fn clear_senses(db: &DbPool) -> Result<(), Error> {
    use crate::schema::sense::dsl::*;
    diesel::delete(sense).execute_async(db).await?;
    Ok(())
}
