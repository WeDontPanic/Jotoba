use search::{
    self,
    query::UserSettings,
    query_parser::{QueryParser, QueryType},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryStruct {
    #[serde(rename = "t")]
    pub search_type: Option<QueryType>,
    #[serde(rename = "i")]
    pub word_index: Option<usize>,
    #[serde(rename = "p", default = "default_page")]
    pub page: usize,

    #[serde(skip_serializing, skip_deserializing)]
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
        )
    }
}

#[inline]
fn default_page() -> usize {
    1
}
