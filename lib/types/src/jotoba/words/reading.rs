use serde::{Deserialize, Serialize};

use super::Dict;

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
}

// Jotoba intern only features
#[cfg(feature = "jotoba_intern")]
impl Reading {
    /// Return `true` if reading represents a katakana only word
    #[inline]
    pub fn is_katakana(&self) -> bool {
        use japanese::JapaneseExt;
        self.kana.reading.is_katakana() && self.kanji.is_none()
    }
}

pub struct ReadingIter<'a> {
    reading: &'a Reading,
    allow_kana: bool,
    did_kanji: bool,
    did_kana: bool,
    alternative_pos: u8,
}

impl<'a> ReadingIter<'a> {
    #[inline]
    fn new(reading: &'a Reading, allow_kana: bool) -> Self {
        Self {
            reading,
            allow_kana,
            did_kana: false,
            did_kanji: false,
            alternative_pos: 0,
        }
    }
}

impl<'a> Iterator for ReadingIter<'a> {
    type Item = &'a Dict;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.did_kana && self.allow_kana {
            self.did_kana = true;
            return Some(&self.reading.kana);
        }
        if !self.did_kanji && self.reading.kanji.is_some() {
            self.did_kanji = true;
            return Some(self.reading.kanji.as_ref().unwrap());
        }
        let i = self
            .reading
            .alternative
            .get(self.alternative_pos as usize)?;
        self.alternative_pos += 1;
        Some(i)
    }
}
