use types::jotoba::words::Word;

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
    pub fn irregular_ichidan(&'a self) -> impl Iterator<Item = &'a Word> {
        self.storage
            .irregular_ichidan
            .iter()
            .copied()
            .filter_map(|seq| self.by_sequence(seq))
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
    pub fn by_jlpt(&self, jlpt: u8) -> impl Iterator<Item = &'_ Word> {
        self.storage
            .jlpt_word_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_sequence(*i))
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.storage.count()
    }
}
