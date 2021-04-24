use std::cmp::Ordering;

use crate::{japanese::JapaneseExt, models::name::Name, search::utils::levenshtein_cmp};
use levenshtein::levenshtein;

/// Represents the ordering for name search
/// result based on non-native search-input
pub(crate) struct ByTranscription<'a> {
    query: &'a str,
}

impl<'a> ByTranscription<'a> {
    pub(crate) fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Sort native words results
    pub(crate) fn sort(&self, vec: &mut Vec<Name>) {
        vec.sort_by(|a, b| self.native_words(a, b))
    }

    /// Returns an Ordering variant based on the input items
    fn native_words(&self, this: &Name, other: &Name) -> Ordering {
        let this_le = levenshtein(&this.transcription, self.query);
        let other_le = levenshtein(&other.transcription, self.query);
        levenshtein_cmp(this_le, other_le)
    }
}

/// Represents the ordering for name search
/// result based on native search-input
pub(crate) struct ByNative<'a> {
    query: &'a str,
}

impl<'a> ByNative<'a> {
    pub(crate) fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Sort native words results
    pub(crate) fn sort(&self, vec: &mut Vec<Name>) {
        vec.sort_by(|a, b| self.native_words(a, b))
    }

    /// Returns an Ordering variant based on the input items
    fn native_words(&self, this: &Name, other: &Name) -> Ordering {
        let order = if self.query.is_kanji() {
            self.kanji_check(this, other)
        } else if self.query.is_kana() {
            self.kana_check(this, other)
        } else {
            Ordering::Equal
        };

        if order == Ordering::Equal {
            if self.query.is_kana() {
                if this.kanji.is_none() && other.kanji.is_some() {
                    return Ordering::Less;
                } else if this.kanji.is_some() && other.kanji.is_none() {
                    return Ordering::Greater;
                }
            }
        }

        order
    }

    fn kanji_check(&self, this: &Name, other: &Name) -> Ordering {
        let this_le = levenshtein(&this.kanji.as_ref().unwrap_or(&this.kana), self.query);
        let other_le = levenshtein(&other.kanji.as_ref().unwrap_or(&other.kana), self.query);
        levenshtein_cmp(this_le, other_le)
    }

    fn kana_check(&self, this: &Name, other: &Name) -> Ordering {
        let this_le = levenshtein(&this.kana, self.query);
        let other_le = levenshtein(&other.kana, self.query);
        levenshtein_cmp(this_le, other_le)
    }
}
