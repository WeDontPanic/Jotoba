use std::str::FromStr;

use search::{self, query::UserSettings, query_parser::QueryParser};
use serde::{Deserialize, Deserializer};
use types::jotoba::{languages::Language, search::QueryType};

#[derive(Deserialize)]
pub struct QueryStruct {
    #[serde(rename = "t")]
    pub search_type: Option<QueryType>,
    #[serde(rename = "i")]
    pub word_index: Option<usize>,
    #[serde(rename = "p", default = "default_page")]
    pub page: usize,

    #[serde(default, rename = "l", deserialize_with = "deserialize_lang")]
    pub lang_overwrite: Option<Language>,

    #[serde(skip)]
    pub query_str: String,
}

impl QueryStruct {
    /// Adjusts the search query trim and map empty search queries to Option::None.
    /// Ensures `search_type` is always 'Some()'
    pub fn adjust(&self, query_str: String) -> Self {
        let query_str = query_str.trim().to_string();

        let page = if self.page == 0 {
            default_page()
        } else {
            self.page
        };

        QueryStruct {
            query_str,
            search_type: Some(self.search_type.unwrap_or_default()),
            page,
            word_index: self.word_index,
            lang_overwrite: self.lang_overwrite,
        }
    }

    /// Returns a [`QueryParser`] of the query
    #[inline]
    pub fn as_query_parser(&self, user_settings: UserSettings) -> QueryParser {
        QueryParser::new(
            self.query_str.clone(),
            self.search_type.unwrap_or_default(),
            user_settings,
            self.page,
            self.word_index.unwrap_or_default(),
            true,
            self.lang_overwrite,
        )
    }
}

#[inline]
fn default_page() -> usize {
    1
}

/// Query format for js fallback queries of the format http://127.0.0.1:8080/search?t=0&s=world
/// instead of the query being an url parameter
#[derive(Deserialize)]
pub struct NoJSQueryStruct {
    #[serde(rename = "s")]
    pub query: String,
    #[serde(rename = "t")]
    pub search_type: Option<QueryType>,
    #[serde(rename = "i")]
    pub word_index: Option<usize>,
    #[serde(rename = "p", default = "default_page")]
    pub page: usize,

    #[serde(default, rename = "l", deserialize_with = "deserialize_lang")]
    pub lang_overwrite: Option<Language>,
}

impl NoJSQueryStruct {
    /// Converts a NoJSQueryStruct into a QueryStruct and the query string
    pub(crate) fn to_query_struct(self) -> (QueryStruct, String) {
        let query_struct = QueryStruct {
            page: self.page,
            word_index: self.word_index,
            search_type: self.search_type,
            query_str: String::new(),
            lang_overwrite: self.lang_overwrite,
        };

        (query_struct, self.query)
    }
}

/// Deserializes a field into a Option<Language>. None if invalid lang-str or Deserializing str
/// failed
fn deserialize_lang<'de, D>(s: D) -> Result<Option<Language>, D::Error>
where
    D: Deserializer<'de>,
{
    return Ok(Language::from_str(&String::deserialize(s)?).ok());
}
