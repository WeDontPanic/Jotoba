use std::path::Path;

use config::Config;
use log::info;
use once_cell::sync::OnceCell;
use vector_space_model::DefaultMetadata;

use crate::engine::document::SentenceDocument;

// Shortcut for type of index
pub(super) type Index = vector_space_model::Index<SentenceDocument, DefaultMetadata>;

// In-memory storage for japanese sentence index
pub(super) static INDEX: OnceCell<Index> = OnceCell::new();

/// Load japanese sentence index
pub fn load(config: &Config) {
    let file = Path::new(config.get_indexes_source()).join("sentences_jp_index");
    let index = Index::open(file).expect("Could not load japanese sentence index");
    info!("Loaded japanese sentence index");
    INDEX.set(index).ok();
}

/// Returns the loaded japanese sentence index
#[inline]
pub(crate) fn get() -> &'static Index {
    // Safety:
    // We don't write to `INDEX` after loading it one time at the startup. Jotoba panics if it
    // can't load this index, so until a `get()` call gets reached, `INDEX` is always set to a
    // value.
    unsafe { INDEX.get_unchecked() }
}
