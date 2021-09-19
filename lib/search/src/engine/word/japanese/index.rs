use std::path::Path;

use log::info;
use once_cell::sync::OnceCell;
use vector_space_model::DefaultMetadata;

use crate::engine::document::SingleDocument;

// Shortcut for type of index
pub(super) type Index = vector_space_model::Index<SingleDocument, DefaultMetadata>;

// In-memory storage for japanese index
pub(super) static INDEX: OnceCell<Index> = OnceCell::new();

/// Load japanese index
pub fn load<P: AsRef<Path>>(path: P) {
    let file = path.as_ref().join("jp_index");
    let index = Index::open(file).expect("Could not load japanese index");
    info!("Loaded japanese index");
    INDEX.set(index).unwrap();
}

/// Returns the loaded japanese index
#[inline]
pub(crate) fn get() -> &'static Index {
    INDEX.get().unwrap()
}
