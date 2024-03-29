use types::jotoba::{
    search::SearchTarget,
    sentences,
    words::{misc::Misc, part_of_speech::PosSimple},
};

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Tag {
    // Producer tags
    PartOfSpeech(PosSimple),
    Misc(Misc),
    Jlpt(u8),
    GenkiLesson(u8),
    Katakana,
    SentenceTag(sentences::Tag),
    IrregularIruEru,

    // Non producer
    SearchType(SearchTarget),
    Hidden,
}

impl Tag {
    /// Returns true if the tag can be used without a query
    #[inline]
    pub fn is_producer(&self) -> bool {
        !self.is_search_type() && !self.is_hidden()
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
    #[inline]
    pub fn is_genki_lesson(&self) -> bool {
        matches!(self, Self::GenkiLesson(..))
    }

    #[inline]
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

    /// Returns `true` if the tag is [`Hidden`].
    ///
    /// [`Hidden`]: Tag::Hidden
    #[must_use]
    pub fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden)
    }

    /// Returns `true` if the tag is [`SentenceTag`].
    ///
    /// [`SentenceTag`]: Tag::SentenceTag
    #[must_use]
    #[inline]
    pub fn is_sentence_tag(&self) -> bool {
        matches!(self, Self::SentenceTag(..))
    }

    #[inline]
    pub fn as_sentence_tag(&self) -> Option<&sentences::Tag> {
        if let Self::SentenceTag(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`Katakana`].
    ///
    /// [`Katakana`]: Tag::Katakana
    #[must_use]
    pub fn is_katakana(&self) -> bool {
        matches!(self, Self::Katakana)
    }
}
