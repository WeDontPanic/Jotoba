use std::{fs::File, io::BufReader, path::Path};

use bktree::BkTree;
use config::Config;
use log::info;
use once_cell::sync::OnceCell;
use vector_space_model::DefaultMetadata;

use crate::engine::document::MultiDocument;

// Shortcut for type of index
pub(super) type Index = vector_space_model::Index<MultiDocument, DefaultMetadata>;

// In-memory storage for foreign name index
pub(super) static INDEX: OnceCell<Index> = OnceCell::new();

// In-memory storage for foreign name index
pub(super) static TERM_TREE: OnceCell<BkTree<String>> = OnceCell::new();

/// Load foreign name index
pub(crate) fn load(config: &Config) {
    load_term_treepath(config);

    let file = Path::new(config.get_indexes_source()).join("name_foreign_index");
    let index = Index::open(file).expect("Could not load foreign name index");
    info!("Loaded foreign name index");
    INDEX.set(index).ok();
}

/// Load foreign name term tree
pub fn load_term_treepath(config: &Config) {
    let path = Path::new(config.get_indexes_source()).join("name_foreign_index.tree");
    let file = File::open(path).expect("Failed to parse name term tree");
    let tt =
        bincode::deserialize_from(BufReader::new(file)).expect("Failed to parse name term tree");
    info!("Loaded name term tree");

    TERM_TREE.set(tt).ok();
}

/// Returns the loaded foreign name index
#[inline]
pub(crate) fn get() -> &'static Index {
    // Safety:
    // We don't write to `INDEX` after loading it one time at the startup. Jotoba panics if it
    // can't load this index, so until a `get()` call gets reached, `INDEX` is always set to a
    // value.
    unsafe { INDEX.get_unchecked() }
}

/// Returns the loaded foreign name index
#[inline]
pub(crate) fn get_term_tree() -> &'static BkTree<String> {
    // Safety:
    // We don't write to `INDEX` after loading it one time at the startup. Jotoba panics if it
    // can't load this index, so until a `get()` call gets reached, `INDEX` is always set to a
    // value.
    unsafe { TERM_TREE.get_unchecked() }
}
