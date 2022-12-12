use super::super::storage::name::NameStorage;
use types::jotoba::names::Name;

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
        self.storage.names.get(&seq_id)
    }

    /// Returns the amount of names
    #[inline]
    pub fn count(&self) -> usize {
        self.storage.names.len()
    }

    /// Returns an iterator over all names
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'a Name> {
        self.storage.names.iter().map(|i| i.1)
    }
}
