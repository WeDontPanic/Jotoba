pub mod name;
pub mod radical;
pub mod sentence;
pub mod suggestions;
pub(crate) mod utils;
pub mod word;

use once_cell::sync::OnceCell;
use std::{error::Error, path::Path};
use {name::NameStore, radical::RadicalStore, sentence::SentenceStore, word::WordStore};

/// In-memory store for all indexes
pub(crate) static INDEX_STORE: OnceCell<IndexStore> = OnceCell::new();

/// Store for all indexes
pub struct IndexStore {
    word: WordStore,
    sentence: SentenceStore,
    name: NameStore,
    radical: RadicalStore,
}

impl IndexStore {
    #[inline(always)]
    pub fn word(&self) -> &WordStore {
        &self.word
    }

    #[inline(always)]
    pub fn sentence(&self) -> &SentenceStore {
        &self.sentence
    }

    #[inline(always)]
    pub fn name(&self) -> &NameStore {
        &self.name
    }

    #[inline(always)]
    pub fn radical(&self) -> &RadicalStore {
        &self.radical
    }

    /// Returns `true` if all indexes are properly loaded
    pub fn check(&self) -> bool {
        self.word.check() && self.sentence.check() && self.name.check() && self.radical.check()
    }
}

/// Returns an IndexStore which can be used to retrieve all indexes
#[inline(always)]
pub fn get() -> &'static IndexStore {
    unsafe { INDEX_STORE.get_unchecked() }
}

/// Loads all indexes
pub fn load<P: AsRef<Path>>(index_folder: P) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if is_loaded() {
        return Ok(true);
    }

    let store = load_raw(index_folder)?;

    if !store.check() {
        return Ok(false);
    }

    INDEX_STORE.set(store).ok();

    Ok(true)
}

pub fn is_loaded() -> bool {
    INDEX_STORE.get().is_some()
}

/// Needed for tests only
pub fn wait() {
    INDEX_STORE.wait();
}

pub fn load_raw<P: AsRef<Path>>(
    index_folder: P,
) -> Result<IndexStore, Box<dyn Error + Send + Sync>> {
    log::debug!("Loading word index");
    let word = word::load(index_folder.as_ref())?;

    log::debug!("Loading sentence index");
    let sentence = sentence::load(index_folder.as_ref())?;

    log::debug!("Loading name index");
    let name = name::load(index_folder.as_ref())?;

    log::debug!("Loading radical index");
    let radical = radical::load(index_folder.as_ref())?;

    Ok(IndexStore {
        word,
        sentence,
        name,
        radical,
    })
}
