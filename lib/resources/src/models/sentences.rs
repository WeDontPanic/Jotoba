use crate::parse::jmdict::languages::Language;
use serde::{Deserialize, Serialize};

/// A single Sentence with multiple translations.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Sentence {
    pub japanese: String,
    pub furigana: String,
    pub translations: Vec<Translation>,
}

/// A Translation for a sentence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub text: String,
    pub language: Language,
}

impl Sentence {
    /// Create a new sentence
    #[inline]
    pub fn new(japanese: String, furigana: String, translations: Vec<Translation>) -> Self {
        Sentence {
            japanese,
            furigana,
            translations,
        }
    }

    /// Returns the kana reading of a sentence
    #[inline]
    pub fn get_kana(&self) -> String {
        japanese::furigana::from_str(&self.furigana)
            .map(|i| i.kana)
            .collect::<String>()
    }

    /// Returns `true` if the sentence contains a translation for `language`
    #[inline]
    pub fn has_translation(&self, language: Language) -> bool {
        self.translations.iter().any(|tr| tr.language == language)
    }
}

impl From<(String, Language)> for Translation {
    #[inline]
    fn from((text, language): (String, Language)) -> Self {
        Self { text, language }
    }
}
