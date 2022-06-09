use std::str::FromStr;
use types::jotoba::words::{misc::Misc, part_of_speech::PosSimple};

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Tag {
    SearchType(SearchTypeTag),
    PartOfSpeech(PosSimple),
    Misc(Misc),
    Jlpt(u8),
    GenkiLesson(u8),
    Hidden,
    IrregularIruEru,
}

/// #words #sentence #name and #kanji hashtags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum SearchTypeTag {
    Kanji,
    Sentence,
    Name,
    Word,
}

impl Tag {
    /// Parse a tag from a string
    pub fn parse_from_str(s: &str) -> Option<Tag> {
        if let Some(tag) = s.to_lowercase().strip_prefix("#") {
            match tag {
                "hidden" | "hide" => return Some(Tag::Hidden),
                "irrichidan" | "irregularichidan" | "irregular-ichidan" => {
                    return Some(Tag::IrregularIruEru)
                }
                _ => (),
            }
        }

        #[allow(irrefutable_let_patterns)]
        if let Some(tag) = Self::parse_genki_tag(s) {
            return Some(tag);
        } else if let Some(tag) = Self::parse_jlpt_tag(s) {
            return Some(tag);
        } else if let Some(tag) = Self::parse_search_type(s) {
            return Some(tag);
        } else {
            match PosSimple::from_str(&s[1..]) {
                Ok(pos) => return Some(Self::PartOfSpeech(pos)),
                _ => return None,
            }
        }
    }

    /// Returns `Some(u8)` if `s` is a valid N-tag
    fn parse_jlpt_tag(s: &str) -> Option<Tag> {
        if s.chars().skip(1).next()?.to_lowercase().next()? != 'n' {
            return None;
        }

        let nr: u8 = s[2..].parse().ok()?;
        (nr > 0 && nr < 6).then(|| Tag::Jlpt(nr))
    }

    /// Returns `Some(u8)` if `s` is a valid genki-tag
    fn parse_genki_tag(s: &str) -> Option<Tag> {
        let e = s.trim().strip_prefix("#")?.trim().to_lowercase();
        if !e.starts_with("genki") {
            return None;
        }

        let nr: u8 = s[6..].parse().ok()?;
        (nr >= 3 && nr <= 23).then(|| Tag::GenkiLesson(nr))
    }

    /// Parse only search type
    fn parse_search_type(s: &str) -> Option<Tag> {
        Some(match s[1..].to_lowercase().as_str() {
            "kanji" => Self::SearchType(SearchTypeTag::Kanji),
            "sentence" | "sentences" => Self::SearchType(SearchTypeTag::Sentence),
            "name" | "names" => Self::SearchType(SearchTypeTag::Name),
            "word" | "words" => Self::SearchType(SearchTypeTag::Word),
            "abbreviation" | "abbrev" => Self::Misc(Misc::Abbreviation),
            _ => return None,
        })
    }

    /// Returns true if the tag is allowed to be used without a query
    #[inline]
    pub fn is_empty_allowed(&self) -> bool {
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
    pub fn as_search_type(&self) -> Option<&SearchTypeTag> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_jlpt_tag_parsing() {
        assert_eq!(Tag::parse_jlpt_tag("#n4"), Some(Tag::Jlpt(4)));
    }

    #[test]
    fn test_parse_genki_tag_parsing() {
        assert_eq!(Tag::parse_genki_tag("#genki3"), Some(Tag::GenkiLesson(3)));
        assert_eq!(Tag::parse_genki_tag("#genki23"), Some(Tag::GenkiLesson(23)));
    }
}
