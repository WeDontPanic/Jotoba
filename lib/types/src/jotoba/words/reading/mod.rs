pub mod iter;

pub use iter::ReadingIter;

use super::Dict;
use jp_utils::JapaneseExt;
use serde::{Deserialize, Serialize};

/// Various readings of a word
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Hash, Eq)]
pub struct Reading {
    pub kana: Dict,
    pub kanji: Option<Dict>,
    pub alternative: Vec<Dict>,
}

impl Reading {
    /// Returns the preferred word-reading of a `Reading`
    #[inline]
    pub fn get_reading(&self) -> &Dict {
        self.kanji.as_ref().unwrap_or(&self.kana)
    }

    /// Returns an iterator over all reading elements
    #[inline]
    pub fn iter(&self, allow_kana: bool) -> ReadingIter<'_> {
        ReadingIter::new(self, allow_kana)
    }

    /// Return `true` if reading represents a katakana only word
    #[inline]
    pub fn is_katakana(&self) -> bool {
        self.kana.reading.is_katakana() && self.kanji.is_none()
    }
}
