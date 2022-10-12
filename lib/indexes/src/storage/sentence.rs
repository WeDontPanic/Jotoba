use super::utils;
use crate::sentences::{ForeignIndex, NativeIndex};
use std::{error::Error, path::Path};

pub const NATIVE_FILE: &str = "sentences_jp_index";
pub const FOREIGN_FILE: &str = "sentences_fg_index";

/// Store for sentence indexes
pub struct SentenceStore {
    native: NativeIndex,
    foreign: ForeignIndex,
}

impl SentenceStore {
    pub(crate) fn new(native: NativeIndex, foreign: ForeignIndex) -> Self {
        Self { foreign, native }
    }

    /// Returns the foreign index for the given language or `None` if not loaded
    #[inline(always)]
    pub fn foreign(&self) -> &ForeignIndex {
        &self.foreign
    }

    /// Returns the japanese sentence index
    #[inline(always)]
    pub fn native(&self) -> &NativeIndex {
        &self.native
    }

    pub(crate) fn check(&self) -> bool {
        true
    }
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<SentenceStore, Box<dyn Error + Send + Sync>> {
    let native = utils::deser_file(path.as_ref(), NATIVE_FILE)?;
    let foreign = utils::deser_file(path.as_ref(), FOREIGN_FILE)?;
    Ok(SentenceStore::new(native, foreign))
}
