use models::name;
use parse::jmnedict::name_type::NameType;
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

impl From<name::Name> for Name {
    fn from(name: name::Name) -> Self {
        Self {
            kana: name.kana,
            kanji: name.kanji,
            transcription: name.transcription,
            name_type: name.name_type,
            xref: name.xref,
        }
    }
}

impl From<Vec<name::Name>> for Response {
    fn from(name: Vec<name::Name>) -> Self {
        let names: Vec<Name> = name.into_iter().map(|i| Name::from(i)).collect();
        Self { names }
    }
}
