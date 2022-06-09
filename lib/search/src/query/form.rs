use types::jotoba::kanji;

/// The form the query was provided in
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Form {
    /// A single word was provided
    SingleWord,
    /// Multiple words were provided
    MultiWords,
    /// Kanji reading based search eg. '気 ケ'
    KanjiReading(kanji::reading::ReadingSearch),
    /// Tag only. Implies query string to be empty
    TagOnly,
    /// Form was not recognized
    Undetected,
}

impl Form {
    #[inline]
    pub fn as_kanji_reading(&self) -> Option<&kanji::reading::ReadingSearch> {
        if let Self::KanjiReading(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the form is [`KanjiReading`].
    #[inline]
    pub fn is_kanji_reading(&self) -> bool {
        matches!(self, Self::KanjiReading(..))
    }

    /// Returns `true` if the form is [`TagOnly`].
    ///
    /// [`TagOnly`]: Form::TagOnly
    #[inline]
    pub fn is_tag_only(&self) -> bool {
        matches!(self, Self::TagOnly)
    }
}

impl Default for Form {
    #[inline]
    fn default() -> Self {
        Self::Undetected
    }
}
