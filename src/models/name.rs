use super::super::schema::name;
use crate::{
    error::Error,
    parse::jmnedict::{name_type::NameType, NameEntry},
    DbPool,
};
use diesel::prelude::*;
use itertools::Itertools;
use tokio_diesel::*;

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
    /// Returns the Name's types in an human readable way
    pub fn get_types_humanized(&self) -> String {
        if let Some(ref n_types) = self.name_type {
            n_types
                .iter()
                // Don't display gendered types here. We need to
                // display genederd  tags in onether div within the template
                .filter_map(|i| (!i.is_gender()).then(|| i.humanized()))
                .join(", ")
        } else {
            String::from("")
        }
    }

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
