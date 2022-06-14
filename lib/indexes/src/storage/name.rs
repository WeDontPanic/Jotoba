use super::utils;
use crate::names::{ForeignIndex, NativeIndex};
use bktree::BkTree;
use std::{error::Error, path::Path};

const FOREIGN_FILE: &str = "name_foreign_index";
const NATIVE_FILE: &str = "name_jp_index";
const FOREIGN_TREE_FILE: &str = "name_foreign_index.tree";

/// Store for name indexes
pub struct NameStore {
    foreign: ForeignIndex,
    native: NativeIndex,

    term_tree: BkTree<String>,
}

impl NameStore {
    pub(crate) fn new(
        foreign: ForeignIndex,
        native: NativeIndex,
        term_tree: BkTree<String>,
    ) -> Self {
        Self {
            foreign,
            native,
            term_tree,
        }
    }

    #[inline(always)]
    pub fn foreign(&self) -> &ForeignIndex {
        &self.foreign
    }

    #[inline(always)]
    pub fn native(&self) -> &NativeIndex {
        &self.native
    }

    #[inline(always)]
    pub fn term_tree(&self) -> &BkTree<String> {
        &self.term_tree
    }

    pub(crate) fn check(&self) -> bool {
        true
    }
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<NameStore, Box<dyn Error + Send + Sync>> {
    let foreign = ForeignIndex::open(path.as_ref().join(FOREIGN_FILE))?;
    let native = NativeIndex::open(path.as_ref().join(NATIVE_FILE))?;
    let term_tree = utils::deser_file(path, FOREIGN_TREE_FILE)?;
    Ok(NameStore::new(foreign, native, term_tree))
}
