use super::super::schema::name;
use crate::{
    error::Error,
    parse::jmnedict::{name_type::NameType, NameEntry},
    DbPool,
};
use diesel::prelude::*;
use tokio_diesel::*;

#[derive(Queryable, Clone, Debug, Default)]
pub struct Name {
    pub id: i32,
    pub sequence: i32,
    pub kana: String,
    pub kanji: Option<String>,
    pub transcription: String,
    pub name_type: Option<NameType>,
    pub xref: Option<String>,
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "name"]
pub struct NewName {
    pub sequence: i32,
    pub kana: String,
    pub kanji: Option<String>,
    pub transcription: String,
    pub name_type: Option<NameType>,
    pub xref: Option<String>,
}

impl From<NameEntry> for NewName {
    fn from(val: NameEntry) -> Self {
        NewName {
            sequence: val.sequence,
            kana: val.kana_element,
            kanji: val.kanji_element,
            transcription: val.transcription,
            name_type: val.name_type,
            xref: val.xref,
        }
    }
}

/// Insert multiple names into the DB
pub async fn insert_names(db: &DbPool, values: Vec<NewName>) -> Result<(), Error> {
    use crate::schema::name::dsl::*;

    diesel::insert_into(name)
        .values(values)
        .execute_async(&db)
        .await?;

    Ok(())
}

/// Clear all name entries
pub async fn clear(db: &DbPool) -> Result<(), Error> {
    use crate::schema::name::dsl::*;
    diesel::delete(name).execute_async(db).await?;
    Ok(())
}

/// Returns Ok(true) if at least one name exists in the Db
pub async fn exists(db: &DbPool) -> Result<bool, Error> {
    use crate::schema::name::dsl::*;
    Ok(name.select(id).limit(1).execute_async(db).await? == 1)
}
