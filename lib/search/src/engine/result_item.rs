use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use types::jotoba::languages::Language;

/// A single item (result) in a set of search results
#[derive(Clone, Copy, Default, Debug)]
pub struct ResultItem<T> {
    pub item: T,
    pub relevance: usize,
    pub language: Option<Language>,
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

    #[inline]
    pub fn new_raw(item: T, relevance: usize, language: Option<Language>) -> Self {
        Self {
            item,
            relevance,
            language,
        }
    }

    /// Create a new ResultItem<T> with a language set
    #[inline]
    pub fn with_language(item: T, relevance: usize, language: Language) -> Self {
        Self::new_raw(item, relevance, Some(language))
    }
}

impl<T: PartialEq> From<T> for ResultItem<T> {
    #[inline]
    fn from(item: T) -> Self {
        ResultItem::new(item, 0)
    }
}

impl<T: PartialEq> From<(T, usize)> for ResultItem<T> {
    #[inline]
    fn from((item, relevance): (T, usize)) -> Self {
        ResultItem::new(item, relevance)
    }
}

impl<T: PartialEq> PartialEq for ResultItem<T> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: PartialEq> Eq for ResultItem<T> {}

impl<T: PartialEq + Hash> Hash for ResultItem<T> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
    }
}

impl<T: PartialEq> PartialOrd for ResultItem<T> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.relevance.cmp(&other.relevance))
    }
}

impl<T: PartialEq> Ord for ResultItem<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.relevance.cmp(&other.relevance)
    }
}
