use std::path::Path;

use bktree::BkTree;
use log::info;
use once_cell::sync::OnceCell;
use vector_space_model::DefaultMetadata;

use crate::engine::document::SingleDocument;

// Shortcut for type of index
pub(super) type Index = vector_space_model::Index<SingleDocument, DefaultMetadata>;

// In-memory storage for japanese index
pub(super) static INDEX: OnceCell<Index> = OnceCell::new();

// In-memory storage for all loaded term trees
pub(super) static TERM_TREE: OnceCell<BkTree<String>> = OnceCell::new();

/// Load japanese index
pub fn load<P: AsRef<Path>>(path: P) {
    load_index(path.as_ref());
    load_term_tree(path);
}

/// Load japanese index
pub fn load_index<P: AsRef<Path>>(path: P) {
    let file = path.as_ref().join("jp_index");
    let index = Index::open(file).expect("Could not load japanese index");

    let vecs = index.get_vector_store().len();
    let terms = index.get_indexer().size();
    info!("Loaded japanese index ({} terms, {} vectors)", terms, vecs);

    INDEX.set(index).ok();
}

/// Load japanese index
pub fn load_term_tree<P: AsRef<Path>>(_path: P) {
    // let file = path.as_ref().join("jp_index.tree");

    //let file = File::open(file).expect("Failed to parse japanese term tree");
    //let tt = bincode::deserialize_from(file).expect("Failed to parse japanese term tree");
    //info!("Loaded japanese term tree");

    //TERM_TREE.set(tt).ok();
}

/// Returns the loaded japanese index
#[inline]
pub fn get() -> &'static Index {
    unsafe { INDEX.get_unchecked() }
}
