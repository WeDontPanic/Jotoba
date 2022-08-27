use super::utils;
use crate::names::{ForeignIndex, NativeIndex};
use std::{error::Error, path::Path};

const FOREIGN_FILE: &str = "name_foreign_index";
const NATIVE_FILE: &str = "name_jp_index";

/// Store for name indexes
pub struct NameStore {
    foreign: ForeignIndex,
    native: NativeIndex,
}

impl NameStore {
    pub(crate) fn new(foreign: ForeignIndex, native: NativeIndex) -> Self {
        Self { foreign, native }
    }

    #[inline(always)]
    pub fn foreign(&self) -> &ForeignIndex {
        &self.foreign
    }

    #[inline(always)]
    pub fn native(&self) -> &NativeIndex {
        &self.native
    }

    pub(crate) fn check(&self) -> bool {
        true
    }
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<NameStore, Box<dyn Error + Send + Sync>> {
    let foreign = utils::deser_file(path.as_ref(), FOREIGN_FILE)?;
    let native = utils::deser_file(path.as_ref(), NATIVE_FILE)?;
    Ok(NameStore::new(foreign, native))
}
