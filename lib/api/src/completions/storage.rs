use std::{error::Error, fs::File, io::BufReader, path::Path};

use config::Config;
use log::info;
use once_cell::sync::OnceCell;
use search::suggestions::{store_item, TextSearch};
use serde::{Deserialize, Deserializer};

use super::WordPair;

/// In-memory storage for native name suggestions
pub(crate) static NAME_NATIVE: OnceCell<TextSearch<Vec<NameNative>>> = OnceCell::new();

/// In-memory storage for name transcriptions suggestions
pub(crate) static NAME_TRANSCRIPTIONS: OnceCell<TextSearch<Vec<NameTranscription>>> =
    OnceCell::new();

/// In-memory storage for kanji meaning suggestions
pub(crate) static K_MEANING_SUGGESTIONS: OnceCell<TextSearch<Vec<KanjiMeaningSuggestionItem>>> =
    OnceCell::new();

/// Load all available suggestions
pub fn load_suggestions(config: &Config) -> Result<(), Box<dyn Error>> {
    rayon::scope(|s| {
        s.spawn(|_| {
            if let Err(err) = load_meaning_suggestions(config) {
                eprintln!("Error loading meaning suggestions {}", err);
            }
        });
        s.spawn(|_| {
            if let Err(err) = load_name_transcriptions(config) {
                eprintln!("Error loading name suggestions {}", err);
            }
        });
        s.spawn(|_| {
            if let Err(err) = load_native_names(config) {
                eprintln!("Error loading name suggestions {}", err);
            }
        });
    });
    Ok(())
}

/// A single suggestion item for kanji meanings
#[derive(Deserialize)]
pub struct KanjiMeaningSuggestionItem {
    pub meaning: String,
    pub literal: char,
    #[serde(deserialize_with = "eudex_deser")]
    pub hash: eudex::Hash,
    pub score: i32,
}

/// A suggestion Item for transcribed a name
#[derive(Deserialize)]
pub struct NameTranscription {
    pub name: String,
    #[serde(deserialize_with = "eudex_deser")]
    pub hash: eudex::Hash,
}

/// A suggestion Item for a name item in japanese
#[derive(Deserialize)]
pub struct NameNative {
    pub name: String,
}

impl store_item::Item for KanjiMeaningSuggestionItem {
    #[inline]
    fn get_text(&self) -> &str {
        &self.meaning
    }

    #[inline]
    fn get_hash(&self) -> eudex::Hash {
        self.hash
    }

    #[inline]
    fn ord(&self) -> usize {
        self.score as usize
    }
}

impl Into<WordPair> for &KanjiMeaningSuggestionItem {
    #[inline]
    fn into(self) -> WordPair {
        WordPair {
            primary: self.meaning.clone(),
            secondary: Some(self.literal.to_string()),
        }
    }
}

impl store_item::Item for NameTranscription {
    #[inline]
    fn get_text(&self) -> &str {
        &self.name
    }

    #[inline]
    fn get_hash(&self) -> eudex::Hash {
        self.hash
    }
}

impl Into<WordPair> for &NameTranscription {
    #[inline]
    fn into(self) -> WordPair {
        WordPair {
            primary: self.name.clone(),
            ..Default::default()
        }
    }
}

impl store_item::Item for NameNative {
    #[inline]
    fn get_text(&self) -> &str {
        &self.name
    }
}

impl Into<WordPair> for &NameNative {
    #[inline]
    fn into(self) -> WordPair {
        WordPair {
            primary: self.name.clone(),
            ..Default::default()
        }
    }
}

/// Load kanji meaning suggestion file into memory
fn load_meaning_suggestions(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("kanji_meanings");
    if !file.exists() {
        info!("Kanji-meaning suggestion file does not exists");
        return Ok(());
    }

    let kanji_items: Vec<KanjiMeaningSuggestionItem> =
        bincode::deserialize_from(BufReader::new(File::open(file)?))?;

    K_MEANING_SUGGESTIONS.set(TextSearch::new(kanji_items)).ok();

    info!("Loaded kanji meaning suggestion file");

    Ok(())
}

/// Load native name suggestions
fn load_native_names(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("names_native");
    if !file.exists() {
        info!("Native name suggestion file does not exists");
        return Ok(());
    }

    let items: Vec<NameNative> = bincode::deserialize_from(BufReader::new(File::open(file)?))?;

    NAME_NATIVE.set(TextSearch::new(items)).ok();

    info!("Loaded native name suggestion file");

    Ok(())
}

/// Load name transcription suggestions
fn load_name_transcriptions(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("names_trans");
    if !file.exists() {
        info!("Name transcription suggestion file does not exists");
        return Ok(());
    }

    let items: Vec<NameTranscription> =
        bincode::deserialize_from(BufReader::new(File::open(file)?))?;

    NAME_TRANSCRIPTIONS.set(TextSearch::new(items)).ok();

    info!("Loaded name transcriptions suggestion file");

    Ok(())
}

#[inline]
fn eudex_deser<'de, D>(deserializer: D) -> Result<eudex::Hash, D::Error>
where
    D: Deserializer<'de>,
{
    let s: u64 = Deserialize::deserialize(deserializer)?;
    Ok(eudex::Hash::from(s))
}
