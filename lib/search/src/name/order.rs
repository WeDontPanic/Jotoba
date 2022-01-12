use std::cmp::Ordering;

use japanese::JapaneseExt;
use levenshtein::levenshtein;
use types::jotoba::{kanji, names::Name};

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
        this_le.cmp(&other_le)
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

        if order == Ordering::Equal && self.query.is_kana() {
            if this.kanji.is_none() && other.kanji.is_some() {
                return Ordering::Less;
            } else if this.kanji.is_some() && other.kanji.is_none() {
                return Ordering::Greater;
            }
        }

        order
    }

    fn kanji_check(&self, this: &Name, other: &Name) -> Ordering {
        let this_le = levenshtein(&this.kanji.as_ref().unwrap_or(&this.kana), self.query);
        let other_le = levenshtein(&other.kanji.as_ref().unwrap_or(&other.kana), self.query);
        this_le.cmp(&other_le)
    }

    fn kana_check(&self, this: &Name, other: &Name) -> Ordering {
        let this_le = levenshtein(&this.kana, self.query);
        let other_le = levenshtein(&other.kana, self.query);
        this_le.cmp(&other_le)
    }
}

/// Represents the ordering for name search
/// result based on native search-input
pub(crate) struct ByKanji<'a> {
    query: &'a str,
    kanji: &'a kanji::ReadingSearch,
}

impl<'a> ByKanji<'a> {
    pub(crate) fn new(query: &'a str, kanji: &'a kanji::ReadingSearch) -> Self {
        Self { query, kanji }
    }

    /// Sort native words results
    pub(crate) fn sort(&self, vec: &mut Vec<Name>) {
        vec.sort_by(|a, b| self.by_kanji(a, b))
    }

    /// Returns an Ordering variant based on the input items
    fn by_kanji(&self, this: &Name, other: &Name) -> Ordering {
        debug_assert!(this.kanji.is_some());
        debug_assert!(other.kanji.is_some());

        if self.exact(this) && !self.exact(other) {
            return Ordering::Less;
        } else if !self.exact(this) && self.exact(other) {
            return Ordering::Greater;
        }

        if self.right_pos(this) && !self.right_pos(other) {
            return Ordering::Less;
        } else if !self.right_pos(this) && self.right_pos(other) {
            return Ordering::Greater;
        }

        this.kanji
            .as_ref()
            .unwrap()
            .cmp(other.kanji.as_ref().unwrap())
    }

    fn exact(&self, n: &Name) -> bool {
        let kanji = str_to_char(n.kanji.as_ref().unwrap());
        if kanji.is_none() {
            return false;
        }

        kanji.unwrap() == self.kanji.literal && n.kana == self.kanji.reading
    }

    fn right_pos(&self, n: &Name) -> bool {
        let kanji = n.kanji.as_ref().unwrap();

        if kanji.ends_with(self.kanji.literal) {
            return n.kana.ends_with(&self.kanji.reading);
        }

        if kanji.starts_with(self.kanji.literal) {
            return n.kana.starts_with(&self.kanji.reading);
        }

        !n.kana.starts_with(&self.kanji.reading) && !n.kana.ends_with(&self.kanji.reading)
    }
}

fn str_to_char(s: &str) -> Option<char> {
    s.chars().next()
}
