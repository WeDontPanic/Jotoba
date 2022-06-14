use super::utils;
use crate::sentences::{ForeignIndex, NativeIndex};
use std::{collections::HashMap, error::Error, path::Path};
use types::jotoba::languages::Language;

const NATIVE_FILE: &str = "sentences_jp_index";
const FOREIGN_PREFIX: &str = "sentences";

/// Store for sentence indexes
pub struct SentenceStore {
    foreign: HashMap<Language, ForeignIndex>,
    native: NativeIndex,
}

impl SentenceStore {
    pub(crate) fn new(foreign: HashMap<Language, ForeignIndex>, native: NativeIndex) -> Self {
        Self { foreign, native }
    }

    /// Returns the foreign index for the given language or `None` if not loaded
    #[inline(always)]
    pub fn foreign(&self, language: Language) -> Option<&ForeignIndex> {
        self.foreign.get(&language)
    }

    /// Returns the japanese sentence index
    #[inline(always)]
    pub fn native(&self) -> &NativeIndex {
        &self.native
    }

    pub(crate) fn check(&self) -> bool {
        utils::check_lang_map(&self.foreign)
    }
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<SentenceStore, Box<dyn Error + Send + Sync>> {
    let native = NativeIndex::open(path.as_ref().join(NATIVE_FILE))?;
    let foreign = load_foreign(path)?;
    Ok(SentenceStore::new(foreign, native))
}

fn load_foreign<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<Language, ForeignIndex>, Box<dyn Error + Send + Sync>> {
    utils::load_by_language(path, FOREIGN_PREFIX, |p| {
        let file_name = p.file_name().and_then(|i| i.to_str()).unwrap();
        if file_name == NATIVE_FILE || !file_name.ends_with("_index") {
            return Ok(None);
        }

        let index = ForeignIndex::open(p)?;
        let lang = index.get_metadata().language;
        Ok(Some((lang, index)))
    })
}
