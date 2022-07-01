use types::jotoba::kanji;

/// The form the query was provided in
#[derive(Debug, Clone, PartialEq, Hash, Default)]
pub enum Form {
    /// A single word was provided
    SingleWord,

    /// Multiple words were provided
    MultiWords,

    /// Kanji reading based search eg. '気 ケ'
    KanjiReading(kanji::reading::ReadingSearch),

    /// Tag only. Implies query string to be empty
    TagOnly,

    /// Sequence Search
    Sequence(u32),

    /// Form was not recognized
    #[default]
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
