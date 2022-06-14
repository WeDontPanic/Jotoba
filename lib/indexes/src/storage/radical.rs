use super::utils;
use crate::radical::RadicalIndex;
use std::{error::Error, path::Path};

const RAD_INDEX_FILE: &str = "radical_index";

/// Store for radical indexes
pub struct RadicalStore {
    rad_index: RadicalIndex,
}

impl RadicalStore {
    pub(crate) fn new(rad_index: RadicalIndex) -> Self {
        Self { rad_index }
    }

    #[inline]
    pub fn rad_index(&self) -> &RadicalIndex {
        &self.rad_index
    }

    /// Returns true if data is valid
    pub(crate) fn check(&self) -> bool {
        !self.rad_index.meaning_map.is_empty()
    }
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<RadicalStore, Box<dyn Error + Send + Sync>> {
    let index = utils::deser_file(path, RAD_INDEX_FILE)?;
    let store = RadicalStore::new(index);
    Ok(store)
}
