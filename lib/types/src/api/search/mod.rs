pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use serde::Deserialize;

use crate::jotoba::languages::Language;

/// An Search API payload
#[derive(Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "query")]
    pub query_str: String,

    #[serde(default)]
    pub language: Language,

    #[serde(default)]
    pub no_english: bool,
}
