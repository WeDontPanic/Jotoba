use types::jotoba::names::Name;

use super::super::storage::name::NameStorage;

#[derive(Clone, Copy)]
pub struct NameRetrieve<'a> {
    storage: &'a NameStorage,
}

impl<'a> NameRetrieve<'a> {
    #[inline(always)]
    pub(crate) fn new(storage: &'a NameStorage) -> Self {
        NameRetrieve { storage }
    }

    /// Get a name by its sequence id
    #[inline]
    pub fn by_sequence(&self, seq_id: u32) -> Option<&'a Name> {
        self.storage.names.get(seq_id)
    }

    /// Returns the amount of names
    #[inline]
    pub fn count(&self) -> usize {
        self.storage.names.len()
    }
}
