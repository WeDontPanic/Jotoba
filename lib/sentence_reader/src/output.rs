use crate::sentence::part::Part;

/// Result of a sentence/inflection analysis
#[derive(Debug, Clone)]
pub enum ParseResult {
    Sentence(Sentence),
    InflectedWord(Part),
    None,
}

impl ParseResult {
    /// Returns `true` if the parse result is [`Sentence`].
    ///
    /// [`Sentence`]: ParseResult::Sentence
    #[inline]
    pub fn is_sentence(&self) -> bool {
        matches!(self, Self::Sentence(..))
    }

    /// Returns `true` if the parse result is [`InflectedWord`].
    ///
    /// [`InflectedWord`]: ParseResult::InflectedWord
    #[inline]
    pub fn is_inflected_word(&self) -> bool {
        matches!(self, Self::InflectedWord(..))
    }

    /// Returns `true` if the parse result is [`None`].
    ///
    /// [`None`]: ParseResult::None
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    #[inline]
    pub fn as_sentence(&self) -> Option<&Sentence> {
        if let Self::Sentence(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_inflected_word(&self) -> Option<&Part> {
        if let Self::InflectedWord(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/// A split sentence
#[derive(Debug, Clone, PartialEq)]
pub struct Sentence {
    parts: Vec<Part>,
}

impl Sentence {
    #[inline]
    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }

    /// Returns word at `pos`
    #[inline]
    pub fn get_at(&self, pos: usize) -> Option<&Part> {
        self.parts.get(pos)
    }

    /// Returns word at `pos`
    #[inline]
    pub fn get_at_mut(&mut self, pos: usize) -> Option<&mut Part> {
        self.parts.get_mut(pos)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Part> {
        self.parts.iter_mut()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Part> {
        self.parts.iter()
    }

    /// returns amount of words
    #[inline]
    pub fn word_count(&self) -> usize {
        self.parts.len()
    }

    /// Returns all parts owned
    #[inline]
    pub fn into_parts(self) -> Vec<Part> {
        self.parts
    }
}
