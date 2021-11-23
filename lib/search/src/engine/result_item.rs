use resources::parse::jmdict::languages::Language;
use std::{cmp::Ordering, hash::Hash};

/// A single item (result) in a set of search results
#[derive(Clone, Copy, Default, Debug)]
pub struct ResultItem<T: PartialEq> {
    pub item: T,
    pub relevance: usize,
    pub language: Option<Language>,
}

impl<T: PartialEq> PartialEq for ResultItem<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item && self.language == other.language
    }
}

impl<T: PartialEq + Hash> std::hash::Hash for ResultItem<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.item.hash(state);
        //self.language.hash(state);
    }
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

impl<T: PartialEq> AsRef<T> for ResultItem<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.item
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
