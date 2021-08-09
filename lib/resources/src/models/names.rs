use crate::parse::jmnedict::name_type::NameType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Name {
    pub sequence: u32,
    pub kana: String,
    pub kanji: Option<String>,
    pub transcription: String,
    pub name_type: Option<Vec<NameType>>,
    pub xref: Option<String>,
}
