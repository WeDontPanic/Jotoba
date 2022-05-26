use intmap::IntMap;
use serde::{Deserialize, Serialize};
use types::jotoba::names::Name;

/// Storage containing all data related to names
#[derive(Serialize, Deserialize, Default)]
pub struct NameStorage {
    /// Index mapping name id to its `Name` value
    names: IntMap<Name>,
}

impl NameStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
