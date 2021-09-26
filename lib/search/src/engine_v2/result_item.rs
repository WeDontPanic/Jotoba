use std::cmp::Ordering;

use resources::parse::jmdict::languages::Language;

/// A single item (result) in a set of search results
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct ResultItem<T: PartialEq> {
    pub item: T,
    pub relevance: usize,
    pub language: Option<Language>,
}

impl<T: PartialEq> PartialOrd for ResultItem<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.relevance.cmp(&other.relevance))
    }
}

impl<T: PartialEq> Eq for ResultItem<T> {}

impl<T: PartialEq> Ord for ResultItem<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.relevance.cmp(&other.relevance)
    }
}

impl<T: PartialEq> ResultItem<T> {
    /// Create a new ResultItem<T>
    #[inline]
    pub fn new(item: T, relevance: usize) -> Self {
        Self {
            item,
            relevance,
            language: None,
        }
    }

    /// Create a new ResultItem<T> with a language set
    #[inline]
    pub fn with_language(item: T, relevance: usize, language: Language) -> Self {
        Self {
            item,
            relevance,
            language: Some(language),
        }
    }
}
