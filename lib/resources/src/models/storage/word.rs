use crate::models::words::Word;

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
    pub fn by_sequence(&self, seq_id: u32) -> Option<&Word> {
        self.storage.dict_data.words.get(&seq_id)
    }
}
