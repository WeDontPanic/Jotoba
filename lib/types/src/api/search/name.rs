use serde::{Deserialize, Serialize};

use crate::jotoba::names::name_type::NameType;

#[derive(Serialize, Deserialize)]
pub struct Response {
    names: Vec<Name>,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    pub kana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<String>,
    pub transcription: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_type: Option<Vec<NameType>>,
}

impl From<&crate::jotoba::names::Name> for Name {
    #[inline]
    fn from(name: &crate::jotoba::names::Name) -> Self {
        Self {
            kana: name.kana.clone(),
            kanji: name.kanji.clone(),
            transcription: name.transcription.clone(),
            name_type: name.name_type.clone(),
        }
    }
}

impl From<Vec<&crate::jotoba::names::Name>> for Response {
    #[inline]
    fn from(name: Vec<&crate::jotoba::names::Name>) -> Self {
        let names: Vec<Name> = name.into_iter().map(Name::from).collect();
        Self { names }
    }
}
