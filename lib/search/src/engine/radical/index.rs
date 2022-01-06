use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use bktree::BkTree;
use config::Config;
use log::info;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use types::jotoba::kanji::SearchRadicalInfo;

/// Radicals indexed by its meanings
#[derive(Serialize, Deserialize)]
pub struct RadicalIndex {
    pub meaning_map: HashMap<String, Vec<SearchRadicalInfo>>,
    pub term_tree: BkTree<String>,
}

pub(super) static RADICAL_INDEX: OnceCell<RadicalIndex> = OnceCell::new();

/// Load the radical index
pub(crate) fn load(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(Path::new(config.get_indexes_source()).join("radical_index"))?;
    let index: RadicalIndex = bincode::deserialize_from(BufReader::new(file))?;
    RADICAL_INDEX.set(index).ok();
    info!("Loaded radical index");
    Ok(())
}

/// Returns the radical index
pub fn get_index() -> &'static RadicalIndex {
    // Safety: This value never gets written and only set once at startup
    unsafe { RADICAL_INDEX.get_unchecked() }
}

impl RadicalIndex {
    /// Returns `true` if the index contains `term`
    #[inline]
    pub fn has_term(&self, term: &str) -> bool {
        self.meaning_map.contains_key(term)
    }

    /// Returns `SearchRadicalInfo` from the index by its term or `None` if term is not found
    pub fn get(&self, term: &str) -> Option<&Vec<SearchRadicalInfo>> {
        self.meaning_map.get(term)
    }
}
