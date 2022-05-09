use types::jotoba::words::Word;

use super::ResourceStorage;

#[derive(Clone, Copy)]
pub struct WordRetrieve<'a> {
    storage: &'a ResourceStorage,
}

impl<'a> WordRetrieve<'a> {
    #[inline]
    pub(super) fn new(storage: &'a ResourceStorage) -> Self {
        WordRetrieve { storage }
    }

    /// Get a word by its sequence id
    #[inline]
    pub fn by_sequence(&self, seq_id: u32) -> Option<&'a Word> {
        self.storage.dict_data.word_data.words.get(seq_id as u64)
    }

    /// returns an iterator over all irregular ichidan words
    pub fn irregular_ichidan(&'a self) -> impl Iterator<Item = &'a Word> {
        self.storage
            .dict_data
            .word_data
            .irregular_ichidan
            .iter()
            .copied()
            .filter_map(|seq| self.by_sequence(seq))
    }

    /// returns an iterator over all irregular ichidan words
    pub fn irregular_ichidan_len(&self) -> usize {
        self.storage.dict_data.word_data.irregular_ichidan.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Word> {
        self.storage.dict_data.word_data.words.iter().map(|i| i.1)
    }
}
