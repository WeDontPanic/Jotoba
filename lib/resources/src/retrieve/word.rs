use super::super::storage::word::WordStorage;
use types::jotoba::words::{misc::Misc, part_of_speech::PosSimple, Word};

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

    /// Returns an iterator over all words
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'a Word> {
        self.storage.words.iter().map(|i| i.1)
    }

    /// returns an iterator over all irregular ichidan words
    pub fn irregular_ichidan<'b>(
        &'b self,
    ) -> impl Iterator<Item = &'a Word> + 'b + DoubleEndedIterator {
        self.storage
            .irregular_ichidan
            .iter()
            .copied()
            .filter_map(|seq| self.by_sequence(seq))
    }

    /// Returns the amount of irregular ichidan words that have been indexed
    #[inline]
    pub fn irregular_ichidan_len(&self) -> usize {
        self.storage.irregular_ichidan.len()
    }

    /// Returns an iterator over all words with given `jlpt` level
    #[inline]
    pub fn by_jlpt<'b>(
        &'b self,
        jlpt: u8,
    ) -> impl Iterator<Item = &'a Word> + 'b + DoubleEndedIterator {
        self.storage
            .jlpt_word_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_sequence(*i))
    }

    /// Returns the amount of words indexed for given jlpt level
    #[inline]
    pub fn jlpt_len(&self, jlpt: u8) -> Option<usize> {
        self.storage.jlpt_word_map.get(&jlpt).map(|i| i.len())
    }

    /// Returns an iterator over all words with given `misc`
    #[inline]
    pub fn by_pos_simple<'b>(
        &'b self,
        pos: PosSimple,
    ) -> impl Iterator<Item = &'a Word> + 'b + DoubleEndedIterator {
        self.storage
            .pos_map
            .get(&(pos as u8))
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_sequence(*i))
    }

    /// Returns the amount of words indexed for `pos`
    #[inline]
    pub fn pos_simple_len(&self, pos: &PosSimple) -> Option<usize> {
        self.storage.pos_map.get(&(*pos as u8)).map(|i| i.len())
    }

    /// Returns an iterator over all words with given `misc`
    #[inline]
    pub fn by_misc<'b>(
        &'b self,
        misc: Misc,
    ) -> impl Iterator<Item = &'a Word> + 'b + DoubleEndedIterator {
        self.storage
            .misc_map
            .get(&(misc as u8))
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_sequence(*i))
    }

    /// Returns the amount of words indexed for misc
    #[inline]
    pub fn misc_len(&self, misc: &Misc) -> Option<usize> {
        self.storage.misc_map.get(&(*misc as u8)).map(|i| i.len())
    }

    /// Returns the total count of words
    #[inline]
    pub fn count(&self) -> usize {
        self.storage.count()
    }
}
