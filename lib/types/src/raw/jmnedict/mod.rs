use crate::jotoba::names::name_type::NameType;

/// An dict entry. Represents one word, phrase or expression
#[derive(Default)]
pub struct NameEntry {
    pub sequence: i32,
    pub kana_element: String,
    pub kanji_element: Option<String>,
    pub transcription: String,
    pub name_type: Option<Vec<NameType>>,
    pub xref: Option<String>,
}
