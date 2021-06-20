use crate::{
    queryable::{prepared_query, FromRow, SQL},
    schema::sense,
    DbConnection,
};
use deadpool_postgres::{tokio_postgres::Row, Pool};
use diesel::prelude::*;
use error::Error;
use parse::jmdict::{
    dialect::Dialect,
    field::Field,
    gtype::GType,
    languages::Language,
    misc::Misc,
    part_of_speech::{PartOfSpeech, PosSimple},
    Entry,
};

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

impl SQL for Sense {
    fn get_tablename() -> &'static str {
        "sense"
    }
}

impl FromRow for Sense {
    fn from_row(row: &tokio_postgres::Row, offset: usize) -> Self
    where
        Self: Sized,
    {
        Self {
            id: row.get(offset + 0),
            sequence: row.get(offset + 1),
            language: row.get(offset + 2),
            gloss_pos: row.get(offset + 3),
            gloss: row.get(offset + 4),
            misc: row.get(offset + 5),
            part_of_speech: row.get(offset + 6),
            dialect: row.get(offset + 7),
            xref: row.get(offset + 8),
            gtype: row.get(offset + 9),
            field: row.get(offset + 10),
            information: row.get(offset + 11),
            antonym: row.get(offset + 12),
            pos_simplified: row.get(offset + 13),
        }
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
pub async fn exists(db: &DbConnection) -> Result<bool, Error> {
    use crate::schema::sense::dsl::*;
    Ok(sense.select((id, sequence)).limit(1).execute(db)? == 1)
}

/// Insert multiple dicts into the database
pub async fn insert_sense(db: &DbConnection, senses: Vec<NewSense>) -> Result<(), Error> {
    use crate::schema::sense::dsl::*;

    diesel::insert_into(sense).values(senses).execute(db)?;

    Ok(())
}

pub async fn short_glosses(
    db: &Pool,
    seq: i32,
    lang: Language,
) -> Result<(i32, Vec<String>), Error> {
    let sql = "SELECT gloss FROM sense WHERE sequence = $1 AND (language = $2 OR language = $3) ORDER BY language desc, id asc LIMIT 5";
    let res: Vec<String> = prepared_query(db, sql, &[&seq, &lang, &Language::default()]).await?;
    Ok((seq, res))
}

/// Clear all sense entries
pub async fn clear_senses(db: &DbConnection) -> Result<(), Error> {
    use crate::schema::sense::dsl::*;
    diesel::delete(sense).execute(db)?;
    Ok(())
}
