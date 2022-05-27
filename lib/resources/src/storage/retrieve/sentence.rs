use types::jotoba::sentences::Sentence;

use super::super::storage::sentence::SentenceStorage;

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
        self.storage.sentences.get(id as u64)
    }

    /// Returns an iterator over all sentences with given `jlpt` level
    #[inline]
    pub fn by_jlpt(&self, jlpt: u8) -> impl Iterator<Item = &'_ Sentence> {
        self.storage
            .jlpt_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .filter_map(move |i| self.by_id(*i))
    }
}
