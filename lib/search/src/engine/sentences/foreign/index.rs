use std::collections::HashMap;

use config::Config;
use log::{error, info};
use once_cell::sync::OnceCell;
use types::jotoba::languages::Language;

use crate::engine::{document::SentenceDocument, metadata::Metadata};

// Shortcut for type of index
pub(super) type Index = vector_space_model::Index<SentenceDocument, Metadata>;

// In-memory storage for foreign name index
pub(super) static INDEXES: OnceCell<HashMap<Language, Index>> = OnceCell::new();

/// Load all available foreign-word indexes into memory
pub(crate) fn load(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // All index files in index source folder
    let index_files = std::fs::read_dir(config.get_indexes_source()).and_then(|i| {
        i.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
    })?;

    let mut map = HashMap::new();

    for index_file in index_files {
        let file_name = index_file.file_name().and_then(|i| i.to_str()).unwrap();
        if !file_name.starts_with("sentences") || file_name == "sentences_jp_index" {
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

/// Returns the loaded foreign name index
#[inline]
pub(crate) fn get(lang: Language) -> Option<&'static Index> {
    // Safety:
    // We don't write to `INDEX` after loading it one time at the startup. Jotoba panics if it
    // can't load this index, so until a `get()` call gets reached, `INDEX` is always set to a
    // value.
    unsafe { INDEXES.get_unchecked() }.get(&lang)
}
