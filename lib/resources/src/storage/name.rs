use std::collections::HashMap;

use super::feature::Feature;
use serde::{Deserialize, Serialize};
use types::jotoba::names::Name;

/// Storage containing all data related to names
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct NameStorage {
    /// Index mapping name id to its `Name` value
    pub names: HashMap<u32, Name>,
}

impl NameStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert names into the NameStorage
    pub fn insert_names(&mut self, names: Vec<Name>) {
        self.names.clear();

        for name in names {
            self.names.insert(name.sequence, name);
        }
    }

    pub fn get_features(&self) -> Vec<Feature> {
        let mut out = vec![];
        if !self.names.is_empty() {
            out.push(Feature::Names);
        }
        out
    }
}
