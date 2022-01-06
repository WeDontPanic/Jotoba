use std::{collections::HashMap, error::Error, io::BufReader, path::Path};

use bktree::BkTree;
use log::{error, info};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use types::jotoba::languages::Language;

use crate::engine::{document::MultiDocument, metadata::Metadata};

// Shortcut for type of index
pub(super) type Index = vector_space_model::Index<MultiDocument, Metadata>;

// In-memory storage for all loaded indexes
pub(super) static INDEXES: OnceCell<HashMap<Language, Index>> = OnceCell::new();

// In-memory storage for all loaded term trees
pub(super) static TERM_TREE: OnceCell<HashMap<Language, BkTree<String>>> = OnceCell::new();

#[derive(Serialize, Deserialize)]
pub struct TermTree {
    pub language: Language,
    pub tree: BkTree<String>,
}

/// Load all available foreign-word indexes into memory
pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    load_index(path.as_ref())?;
    load_term_trees(path)?;
    Ok(())
}

fn load_term_trees<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    // All index files in index source folder
    let tree_files = std::fs::read_dir(path).and_then(|i| {
        i.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
    })?;

    let mut map = HashMap::new();

    for tree_file in tree_files {
        let file_name = tree_file.file_name().and_then(|i| i.to_str()).unwrap();
        if !file_name.starts_with("word_term") {
            continue;
        }

        let file = std::fs::File::open(tree_file)?;
        let tt: TermTree = bincode::deserialize_from(BufReader::new(file))?;

        map.insert(tt.language, tt.tree);

        info!("Loaded term tree file: {:?}", tt.language);
    }

    if map.is_empty() {
        panic!("No index file loaded");
    }

    TERM_TREE.set(map).ok();

    Ok(())
}

fn load_index<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    // All index files in index source folder
    let index_files = std::fs::read_dir(path).and_then(|i| {
        i.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
    })?;

    let mut map = HashMap::new();

    for index_file in index_files {
        let file_name = index_file.file_name().and_then(|i| i.to_str()).unwrap();
        if !file_name.starts_with("word_index") {
            continue;
        }

        let index = match Index::open(&index_file) {
            Ok(index) => index,
            Err(err) => {
                let file = index_file.display();
                error!("Failed to load index \"{}\": {:?}", file, err);
                continue;
            }
        };

        let lang = index.get_metadata().language;
        map.insert(lang, index);

        info!("Loaded index file: {:?}", lang);
    }

    if map.is_empty() {
        panic!("No index file loaded");
    }

    INDEXES.set(map).ok();

    Ok(())
}

/// Retrieve an index of the given language. Returns `None` if there is no index loaded
#[inline]
pub(super) fn get(lang: Language) -> Option<&'static Index> {
    // Safety:
    // We don't write to `INDEX` after loading it one time at the startup. Jotoba panics if it
    // can't load this index, so until a `get()` call gets reached, `INDEX` is always set to a
    // value.
    unsafe { INDEXES.get_unchecked() }.get(&lang)
}

/// Retrieve a term tree of the given language. Returns `None` if there is no index loaded
#[inline]
pub(super) fn get_term_tree(lang: Language) -> Option<&'static BkTree<String>> {
    // Safety:
    // We don't write to `INDEX` after loading it one time at the startup. Jotoba panics if it
    // can't load this index, so until a `get()` call gets reached, `INDEX` is always set to a
    // value.
    unsafe { TERM_TREE.get_unchecked() }.get(&lang)
}
