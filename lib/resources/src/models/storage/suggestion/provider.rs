use crate::{models::storage::ResourceStorage, parse::jmdict::languages::Language};

use super::SuggestionDictionary;

pub struct SuggestionProvider<'a> {
    data: &'a ResourceStorage,
}

impl<'a> SuggestionProvider<'a> {
    /// Create a new SuggestionProvider which can be used to retrieve suggestion dictionaries of
    /// different types
    #[inline]
    pub(crate) fn new(data: &'a ResourceStorage) -> SuggestionProvider {
        Self { data }
    }

    /// Get the suggestion dictionary for japanese words, if available
    #[inline]
    pub fn japanese_words(&self) -> Option<&SuggestionDictionary> {
        self.data.suggestions.as_ref()?.japanese.as_ref()
    }

    /// Get the suggestion dictionary for the given language
    #[inline]
    pub fn foreign_words(&self, lang: Language) -> Option<&SuggestionDictionary> {
        self.data.suggestions.as_ref()?.foregin.get(&lang)
    }
}
