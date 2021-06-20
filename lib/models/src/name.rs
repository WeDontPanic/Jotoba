use crate::{
    queryable::{Deletable, FromRow, SQL},
    schema::name,
    DbConnection,
};
use deadpool_postgres::{tokio_postgres::Row, Pool};
use diesel::prelude::*;
use error::Error;
use parse::jmnedict::{name_type::NameType, NameEntry};

#[derive(Queryable, Clone, Debug, Default)]
pub struct Name {
    pub id: i32,
    pub sequence: i32,
    pub kana: String,
    pub kanji: Option<String>,
    pub transcription: String,
    pub name_type: Option<Vec<NameType>>,
    pub xref: Option<String>,
}

impl SQL for Name {
    fn get_tablename() -> &'static str {
        "name"
    }
}

impl FromRow for Name {
    fn from_row(row: &Row, offset: usize) -> Self
    where
        Self: Sized,
    {
        Self {
            id: row.get(offset + 0),
            sequence: row.get(offset + 1),
            kana: row.get(offset + 2),
            kanji: row.get(offset + 3),
            transcription: row.get(offset + 4),
            name_type: row.get(offset + 5),
            xref: row.get(offset + 6),
        }
    }
}

#[derive(Insertable, Clone, Debug, PartialEq)]
#[table_name = "name"]
pub struct NewName {
    pub sequence: i32,
    pub kana: String,
    pub kanji: Option<String>,
    pub transcription: String,
    pub name_type: Option<Vec<NameType>>,
    pub xref: Option<String>,
}

impl Name {
    /// Return true if name is gendered
    pub fn is_gendered(&self) -> bool {
        self.name_type
            .as_ref()
            .map(|i| i.iter().any(|i| i.is_gender()))
            .unwrap_or(false)
    }

    /// Get the gender name-type if exists
    pub fn get_gender(&self) -> Option<NameType> {
        self.name_type
            .as_ref()
            .and_then(|i| i.iter().find(|i| i.is_gender()).copied())
    }

    /// Returns true if name has at least one non-gender tag
    pub fn has_non_gender_tags(&self) -> bool {
        self.name_type
            .as_ref()
            .map(|i| i.iter().any(|j| !j.is_gender()))
            .unwrap_or(false)
    }
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

impl From<Row> for Name {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            sequence: row.get(1),
            kana: row.get(2),
            kanji: row.get(3),
            transcription: row.get(4),
            name_type: row.get(5),
            xref: row.get(6),
        }
    }
}

/// Insert multiple names into the DB
pub async fn insert_names(db: &DbConnection, values: Vec<NewName>) -> Result<(), Error> {
    use crate::schema::name::dsl::*;

    diesel::insert_into(name).values(values).execute(db)?;

    Ok(())
}

/// Clear all name entries
pub async fn clear(db: &Pool) -> Result<(), Error> {
    Name::delete_all(&db).await?;
    Ok(())
}

/// Returns Ok(true) if at least one name exists in the Db
pub async fn exists(db: &DbConnection) -> Result<bool, Error> {
    use crate::schema::name::dsl::*;
    Ok(name.select(id).limit(1).execute(db)? == 1)
}
