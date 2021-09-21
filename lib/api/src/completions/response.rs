use serde::Serialize;

/// Response struct for suggestion endpoint
#[derive(Serialize, Default)]
pub struct Response {
    pub suggestions: Vec<WordPair>,
    pub suggestion_type: SuggestionType,
}

/// The type of suggestion. `Default` in most cases
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionType {
    /// Default suggestion type
    Default,
    /// Special suggestion type for kanji readings
    KanjiReading,
}

impl Default for SuggestionType {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

/// A word with kana and kanji reading used within [`SuggestionResponse`]
#[derive(Serialize, Default, PartialEq, Eq)]
pub struct WordPair {
    pub primary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}

impl WordPair {
    /// Returns true if [`self`] contains [`reading`]
    pub(crate) fn has_reading(&self, reading: &str) -> bool {
        self.primary == reading
            || self
                .secondary
                .as_ref()
                .map(|i| i == reading)
                .unwrap_or_default()
    }
}
