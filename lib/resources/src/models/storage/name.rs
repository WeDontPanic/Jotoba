use crate::models::names::Name;

use super::ResourceStorage;

#[derive(Clone, Copy)]
pub struct NameRetrieve<'a> {
    storage: &'a ResourceStorage,
}

impl<'a> NameRetrieve<'a> {
    #[inline]
    pub(super) fn new(storage: &'a ResourceStorage) -> Self {
        NameRetrieve { storage }
    }

    /// Get a name by its sequence id
    #[inline]
    pub fn by_sequence(&self, seq_id: u32) -> Option<&Name> {
        self.storage.dict_data.names.get(&seq_id)
    }
}
