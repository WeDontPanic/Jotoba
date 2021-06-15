use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use config::Config;
use itertools::Itertools;
use log::info;
use parse::jmdict::languages::Language;
use search::suggestions::{store_item, SuggestionSearch, TextSearch};

use super::SUGGESTIONS;

/// A single suggestion item which exists of text and its sequence id to be able to assign results
/// to database entries
#[derive(Clone, Debug)]
pub struct SuggestionItem {
    pub text: String,
    pub sequence: i32,
    pub hash: eudex::Hash,
}

impl store_item::Item for SuggestionItem {
    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_hash(&self) -> eudex::Hash {
        self.hash
    }
}

impl FromStr for SuggestionItem {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',').rev();
        let number: i32 = split.next().ok_or(error::Error::ParseError)?.parse()?;
        let text: String = split.rev().join(",");
        Ok(SuggestionItem {
            // generate hash here so lookups will be much faster
            hash: eudex::Hash::new(&text),
            text,
            sequence: number,
        })
    }
}

/// Load Suggestions from suggestion folder into memory
pub fn load_suggestions(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut map = HashMap::new();

    // All items within the configured suggestion directory
    let dir_entries = fs::read_dir(config.get_suggestion_sources()).and_then(|i| {
        i.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()
    })?;

    for entry in dir_entries {
        let entry_name = entry.file_name().unwrap().to_str().unwrap();

        let lang = match Language::from_str(entry_name) {
            Ok(v) => v,
            // Skip files with invalid filename
            Err(_) => {
                info!("Ignoring invalid suggestion-file {}", entry_name);
                continue;
            }
        };

        map.insert(lang, TextSearch::new(load_file(&entry)?));
        info!("Loaded {:?} suggestion file", lang);
    }

    SUGGESTIONS.set(SuggestionSearch::new(map)).ok();
    Ok(())
}

/// Parse a single suggestion file
fn load_file(path: &PathBuf) -> Result<Vec<SuggestionItem>, error::Error> {
    let file = File::open(path)?;

    BufReader::new(file)
        .lines()
        .map(|i| {
            i.map_err(|i| i.into())
                .and_then(|i| SuggestionItem::from_str(&i))
        })
        .collect::<Result<Vec<SuggestionItem>, error::Error>>()
}
