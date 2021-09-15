use std::{collections::HashMap, error::Error};

use crate::engine::document::MultiDocument;

use super::metadata::Metadata;
use config::Config;
use log::{error, info};
use once_cell::sync::OnceCell;
use resources::parse::jmdict::languages::Language;

// Shortcut for type of index
pub(super) type Index = vector_space_model::Index<MultiDocument, Metadata>;

// In-memory storage for all loaded indexes
pub(super) static INDEXES: OnceCell<HashMap<Language, Index>> = OnceCell::new();

/// Load all available foreign-word indexes into memory
pub(crate) fn load(config: &Config) -> Result<(), Box<dyn Error>> {
    // All index files in index source folder
    let index_files = std::fs::read_dir(config.get_indexes_source()).and_then(|i| {
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

    INDEXES.set(map).unwrap();

    Ok(())
}

/// Retrieve an index of the given language. Returns `None` if there is no index loaded
pub(super) fn get(lang: Language) -> Option<&'static Index> {
    INDEXES.get().and_then(|indexes| indexes.get(&lang))
}
