pub mod name_type;

use name_type::NameType;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Name {
    pub sequence: u32,
    pub kana: String,
    pub kanji: Option<String>,
    pub transcription: String,
    pub name_type: Option<Vec<NameType>>,
    pub xref: Option<String>,
}

impl Name {
    /// Return `true` if name is gendered
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

    /// Returns `true` if name has at least one non-gender tag
    pub fn has_non_gender_tags(&self) -> bool {
        self.name_type
            .as_ref()
            .map(|i| i.iter().any(|j| !j.is_gender()))
            .unwrap_or(false)
    }

    #[inline]
    pub fn get_reading(&self) -> &str {
        self.kanji.as_ref().unwrap_or(&self.kana)
    }

    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.kanji.is_some()
    }
}

impl PartialEq for Name {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

impl Eq for Name {}

impl Hash for Name {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sequence.hash(state);
    }
}
