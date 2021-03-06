use super::super::storage::sentence::SentenceStorage;
use types::jotoba::sentences::{tag::Tag, Sentence};

#[derive(Clone, Copy)]
pub struct SentenceRetrieve<'a> {
    storage: &'a SentenceStorage,
}

impl<'a> SentenceRetrieve<'a> {
    #[inline(always)]
    pub(crate) fn new(storage: &'a SentenceStorage) -> Self {
        SentenceRetrieve { storage }
    }

    /// Returns a sentence by its id or `None` if no sentence for the given ID exists
    #[inline]
    pub fn by_id(&self, id: u32) -> Option<&'a Sentence> {
        self.storage.sentences.get(id)
    }

    /// Returns an iterator over all sentences with given `jlpt` level
    #[inline]
    pub fn ids_by_jlpt(&self, jlpt: u8) -> impl Iterator<Item = u32> + 'a {
        self.storage
            .jlpt_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .copied()
    }

    /// Returns an iterator over all sentences with given `tag`
    #[inline]
    pub fn by_tag<'b>(&'b self, tag: &Tag) -> impl Iterator<Item = &'a Sentence> + 'b {
        self.storage
            .tag_map
            .get(tag)
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_id(*i))
    }

    /// Returns an iterator over all sentences with given `jlpt` level
    #[inline]
    pub fn by_jlpt<'b>(&'b self, jlpt: u8) -> impl Iterator<Item = &'a Sentence> + 'b {
        self.storage
            .jlpt_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_id(*i))
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.storage.sentences.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'a Sentence> {
        self.storage.sentences.iter().map(|i| i.1)
    }
}
