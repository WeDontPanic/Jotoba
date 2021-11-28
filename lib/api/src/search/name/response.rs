use resources::{models::names, parse::jmnedict::name_type::NameType};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    names: Vec<Name>,
}

#[derive(Serialize)]
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

impl From<&names::Name> for Name {
    #[inline]
    fn from(name: &names::Name) -> Self {
        Self {
            kana: name.kana.clone(),
            kanji: name.kanji.clone(),
            transcription: name.transcription.clone(),
            name_type: name.name_type.clone(),
            xref: name.xref.clone(),
        }
    }
}

impl From<Vec<&names::Name>> for Response {
    #[inline]
    fn from(name: Vec<&names::Name>) -> Self {
        let names: Vec<Name> = name.into_iter().map(Name::from).collect();
        Self { names }
    }
}
