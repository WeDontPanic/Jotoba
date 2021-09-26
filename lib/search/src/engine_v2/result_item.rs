use std::cmp::Ordering;

use resources::parse::jmdict::languages::Language;

/// A single item in a set of search resulted items
#[derive(Clone, Copy, Default, Debug)]
pub struct ResultItem<T: PartialEq> {
    pub item: T,
    pub relevance: f32,
    pub language: Option<Language>,
}

impl<T: PartialEq> Eq for ResultItem<T> {}

impl<T: PartialEq> PartialEq for ResultItem<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.relevance.eq(&other.relevance) && self.item == other.item
    }
}

impl<T: PartialEq> PartialOrd for ResultItem<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.relevance
            .partial_cmp(&other.relevance)
            .map(|i| i.reverse())
    }
}

impl<T: PartialEq> Ord for ResultItem<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap_or(Ordering::Equal)
    }
}

impl<T: PartialEq> ResultItem<T> {
    #[inline]
    pub fn new(item: T, relevance: f32) -> Self {
        Self {
            item,
            relevance,
            language: None,
        }
    }

    #[inline]
    pub fn with_language(item: T, relevance: f32, language: Language) -> Self {
        Self {
            item,
            relevance,
            language: Some(language),
        }
    }
}
