use crate::jotoba::search::SearchTarget;
use serde::{Deserialize, Serialize};

/// Request payload structure for suggestion endpoint
#[derive(Deserialize, Debug)]
pub struct Request {
    /// The search query to find suggestions for
    pub input: String,

    /// The user configured language
    #[serde(default)]
    pub lang: String,

    /// The search type the input is designed for
    #[serde(default)]
    #[serde(rename = "search_type")]
    pub search_target: SearchTarget,

    #[serde(default)]
    pub radicals: Vec<char>,

    #[serde(default)]
    pub hashtag: bool,
}

/// Response struct for suggestion endpoint
#[derive(Serialize, Deserialize, Default)]
pub struct Response {
    pub suggestions: Vec<WordPair>,
    pub suggestion_type: SuggestionType,
}

impl Response {
    #[inline]
    pub fn new(suggestions: Vec<WordPair>) -> Self {
        Self {
            suggestions,
            suggestion_type: SuggestionType::Default,
        }
    }

    #[inline]
    pub fn with_type(suggestions: Vec<WordPair>, suggestion_type: SuggestionType) -> Self {
        Self {
            suggestions,
            suggestion_type,
        }
    }
}

/// The type of suggestion. `Default` in most cases
#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    /// Default suggestion type
    #[default]
    Default,
    /// Special suggestion type for kanji readings
    KanjiReading,
    /// Hash tag suggestions
    Hashtag,
}

/// A word with kana and kanji reading used within [`SuggestionResponse`]
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug, Hash, Clone)]
pub struct WordPair {
    pub primary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}

#[cfg(feature = "jotoba_intern")]
impl WordPair {
    #[inline]
    pub fn new(primary: String) -> Self {
        Self {
            primary,
            secondary: None,
        }
    }

    #[inline]
    pub fn with_secondary(primary: String, secondary: String) -> Self {
        Self {
            primary,
            secondary: Some(secondary),
        }
    }

    /// Returns true if [`self`] contains [`reading`]
    #[inline]
    pub fn has_reading(&self, reading: &str) -> bool {
        self.primary == reading
            || self
                .secondary
                .as_ref()
                .map(|i| i == reading)
                .unwrap_or_default()
    }

    #[inline]
    pub fn secondary_preferred(&self) -> &String {
        self.secondary.as_ref().unwrap_or(&self.primary)
    }
}

#[cfg(feature = "jotoba_intern")]
impl From<&crate::jotoba::words::Word> for WordPair {
    #[inline]
    fn from(word: &crate::jotoba::words::Word) -> Self {
        let main_reading = word.get_reading().reading.to_owned();
        if word.reading.kanji.is_some() {
            WordPair {
                secondary: Some(main_reading),
                primary: word.reading.kana.reading.clone(),
            }
        } else {
            WordPair {
                primary: main_reading,
                secondary: None,
            }
        }
    }
}
