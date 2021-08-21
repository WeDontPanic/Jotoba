use resources::{models::names, parse::jmnedict::name_type::NameType};
use serde::Serialize;

#[derive(Debug, Serialize, Default)]
pub struct Response {
    names: Vec<Name>,
}

#[derive(Debug, Serialize)]
pub struct Name {
    pub kana: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<String>,
    pub transcription: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_type: Option<Vec<NameType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xref: Option<String>,
}

impl From<names::Name> for Name {
    #[inline]
    fn from(name: names::Name) -> Self {
        Self {
            kana: name.kana,
            kanji: name.kanji,
            transcription: name.transcription,
            name_type: name.name_type,
            xref: name.xref,
        }
    }
}

impl From<Vec<names::Name>> for Response {
    #[inline]
    fn from(name: Vec<names::Name>) -> Self {
        let names: Vec<Name> = name.into_iter().map(|i| Name::from(i)).collect();
        Self { names }
    }
}
