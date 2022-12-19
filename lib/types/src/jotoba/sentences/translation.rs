use crate::jotoba::language::Language;
use serde::{Deserialize, Serialize};

/// A Translation for a sentence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub text: String,
    pub language: Language,
}

impl From<(String, Language)> for Translation {
    #[inline]
    fn from((text, language): (String, Language)) -> Self {
        Self { text, language }
    }
}
