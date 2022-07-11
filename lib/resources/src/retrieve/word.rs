use types::jotoba::words::{misc::Misc, part_of_speech::PosSimple, Word};

use super::super::storage::word::WordStorage;

#[derive(Clone, Copy)]
pub struct WordRetrieve<'a> {
    storage: &'a WordStorage,
}

impl<'a> WordRetrieve<'a> {
    #[inline(always)]
    pub(crate) fn new(storage: &'a WordStorage) -> Self {
        WordRetrieve { storage }
    }

    /// Get a word by its sequence id
    #[inline]
    pub fn by_sequence(&self, seq_id: u32) -> Option<&'a Word> {
        self.storage.words.get(seq_id)
    }

    /// returns an iterator over all irregular ichidan words
    pub fn irregular_ichidan<'b>(&'b self) -> impl Iterator<Item = &'a Word> + 'b {
        self.storage
            .irregular_ichidan
            .iter()
            .copied()
            .filter_map(|seq| self.by_sequence(seq))
            .rev()
    }

    /// returns an iterator over all irregular ichidan words
    pub fn irregular_ichidan_len(&self) -> usize {
        self.storage.irregular_ichidan.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'a Word> {
        self.storage.words.iter().map(|i| i.1)
    }

    /// Returns an iterator over all words with given `jlpt` level
    #[inline]
    pub fn by_jlpt<'b>(&'b self, jlpt: u8) -> impl Iterator<Item = &'a Word> + 'b {
        self.storage
            .jlpt_word_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_sequence(*i))
    }

    /// Returns an iterator over all words with given `misc`
    #[inline]
    pub fn by_pos_simple<'b>(&'b self, pos: PosSimple) -> impl Iterator<Item = &'a Word> + 'b {
        self.storage
            .pos_map
            .get(&(pos as u8))
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_sequence(*i))
    }

    /// Returns an iterator over all words with given `misc`
    #[inline]
    pub fn by_misc<'b>(&'b self, misc: Misc) -> impl Iterator<Item = &'a Word> + 'b {
        self.storage
            .misc_map
            .get(&(misc as u8))
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_sequence(*i))
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.storage.count()
    }
}
