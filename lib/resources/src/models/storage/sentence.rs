use types::jotoba::sentences::Sentence;

use super::ResourceStorage;

#[derive(Clone, Copy)]
pub struct SentenceRetrieve<'a> {
    storage: &'a ResourceStorage,
}

impl<'a> SentenceRetrieve<'a> {
    #[inline]
    pub(super) fn new(storage: &'a ResourceStorage) -> Self {
        SentenceRetrieve { storage }
    }

    /// Returns a sentence by its id or `None` if no sentence for the given ID exists
    #[inline]
    pub fn by_id(&self, id: u32) -> Option<&'a Sentence> {
        self.storage.dict_data.sentences.sentences.get(id as u64)
    }
}
