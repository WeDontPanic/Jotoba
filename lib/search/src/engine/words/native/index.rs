use indexes::words::NativeIndex;
use log::info;
use once_cell::sync::OnceCell;
use std::path::Path;

// In-memory storage for japanese index
pub(super) static INDEX: OnceCell<NativeIndex> = OnceCell::new();

/// Load japanese index
pub fn load<P: AsRef<Path>>(path: P) {
    load_index(path.as_ref());
}

/// Load japanese index
pub fn load_index<P: AsRef<Path>>(path: P) {
    let file = path.as_ref().join("jp_index");
    let index = NativeIndex::open(file).expect("Could not load japanese index");

    let vecs = index.get_vector_store().len();
    let terms = index.get_indexer().len();
    info!("Loaded japanese index ({} terms, {} vectors)", terms, vecs);

    INDEX.set(index).ok();
}

/// Returns the loaded japanese index
#[inline]
pub fn get() -> &'static NativeIndex {
    unsafe { INDEX.get_unchecked() }
}
