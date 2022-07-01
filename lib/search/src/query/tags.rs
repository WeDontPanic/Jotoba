use types::jotoba::{
    search::SearchTarget,
    words::{misc::Misc, part_of_speech::PosSimple},
};

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Tag {
    SearchType(SearchTarget),
    PartOfSpeech(PosSimple),
    Misc(Misc),
    Jlpt(u8),
    GenkiLesson(u8),
    Hidden,
    IrregularIruEru,
}

impl Tag {
    /// Returns true if the tag can be used without a query
    #[inline]
    pub fn is_producer(&self) -> bool {
        self.is_jlpt() || self.is_genki_lesson() || self.is_irregular_iru_eru()
    }

    /// Returns `true` if the tag is [`SearchType`].
    #[inline]
    pub fn is_search_type(&self) -> bool {
        matches!(self, Self::SearchType(..))
    }

    /// Returns `true` if the tag is [`PartOfSpeech`].
    #[inline]
    pub fn is_part_of_speech(&self) -> bool {
        matches!(self, Self::PartOfSpeech(..))
    }

    #[inline]
    pub fn as_search_type(&self) -> Option<&SearchTarget> {
        if let Self::SearchType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_part_of_speech(&self) -> Option<&PosSimple> {
        if let Self::PartOfSpeech(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`Misc`].
    ///
    /// [`Misc`]: Tag::Misc
    #[inline]
    pub fn is_misc(&self) -> bool {
        matches!(self, Self::Misc(..))
    }

    #[inline]
    pub fn as_misc(&self) -> Option<&Misc> {
        if let Self::Misc(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`Jlpt`].
    ///
    /// [`Jlpt`]: Tag::Jlpt
    #[inline]
    pub fn is_jlpt(&self) -> bool {
        matches!(self, Self::Jlpt(..))
    }

    #[inline]
    pub fn as_jlpt(&self) -> Option<u8> {
        if let Self::Jlpt(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`GenkiLesson`].
    ///
    /// [`GenkiLesson`]: Tag::GenkiLesson
    pub fn is_genki_lesson(&self) -> bool {
        matches!(self, Self::GenkiLesson(..))
    }

    pub fn as_genki_lesson(&self) -> Option<u8> {
        if let Self::GenkiLesson(v) = self {
            Some(*v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`IrregularIruEru`].
    ///
    /// [`IrregularIruEru`]: Tag::IrregularIruEru
    pub fn is_irregular_iru_eru(&self) -> bool {
        matches!(self, Self::IrregularIruEru)
    }
}
