use std::path::Path;

use config::Config;
use indexes::sentences::NativeIndex;
use log::info;
use once_cell::sync::OnceCell;

// In-memory storage for japanese sentence index
pub(super) static INDEX: OnceCell<NativeIndex> = OnceCell::new();

/// Load japanese sentence index
pub fn load(config: &Config) {
    let file = Path::new(config.get_indexes_source()).join("sentences_jp_index");
    let index = NativeIndex::open(file).expect("Could not load japanese sentence index");
    info!("Loaded japanese sentence index");
    INDEX.set(index).ok();
}

/// Returns the loaded japanese sentence index
#[inline]
pub fn get() -> &'static NativeIndex {
    // Safety:
    // We don't write to `INDEX` after loading it one time at the startup. Jotoba panics if it
    // can't load this index, so until a `get()` call gets reached, `INDEX` is always set to a
    // value.
    unsafe { INDEX.get_unchecked() }
}
