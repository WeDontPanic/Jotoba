use crate::models::{
    storage::ResourceStorage,
    suggestions::{foreign_words::ForeignSuggestion, native_words::NativeSuggestion},
};

use super::SuggestionDictionary;
use types::jotoba::languages::Language;

pub struct SuggestionProvider<'a> {
    data: &'a ResourceStorage,
}

impl<'a> SuggestionProvider<'a> {
    /// Get the suggestion dictionary for japanese words, if available
    #[inline]
    pub fn japanese_words(&self) -> Option<&SuggestionDictionary<NativeSuggestion>> {
        self.data.suggestions.as_ref()?.japanese.as_ref()
    }

    /// Get the suggestion dictionary for the given language
    #[inline]
    pub fn foreign_words(
        &self,
        lang: Language,
    ) -> Option<&SuggestionDictionary<ForeignSuggestion>> {
        self.data.suggestions.as_ref()?.foregin.get(&lang)
    }
}
