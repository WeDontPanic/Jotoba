use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use autocompletion::index::{basic::BasicIndex, japanese::JapaneseIndex};
use config::Config;
use log::info;
use once_cell::sync::OnceCell;
use types::jotoba::languages::Language;

// Words
pub static JP_WORD_INDEX: OnceCell<JapaneseIndex> = OnceCell::new();
pub static WORD_INDEX: OnceCell<HashMap<Language, BasicIndex>> = OnceCell::new();

/// Kanji meanings
pub(crate) static K_MEANING_SUGGESTIONS: OnceCell<JapaneseIndex> = OnceCell::new();

/// Native (japanese) suggestion index
pub(crate) static NAME_NATIVE: OnceCell<JapaneseIndex> = OnceCell::new();

/// Foreign (transcribed) suggestion index
pub(crate) static NAME_TRANSCRIPTIONS: OnceCell<BasicIndex> = OnceCell::new();

/// Load all available suggestions
pub fn load_suggestions(config: &Config) -> Result<(), Box<dyn Error>> {
    rayon::scope(|s| {
        s.spawn(|_| {
            if let Err(err) = load_words(config) {
                eprintln!("Error loading word suggestions {}", err);
            }
        });
        s.spawn(|_| {
            if let Err(err) = load_meanings(config) {
                eprintln!("Error loading meaning suggestions {}", err);
            }
        });
        s.spawn(|_| {
            if let Err(err) = load_names_foreign(config) {
                eprintln!("Error loading foreign name suggestions {}", err);
            }
        });
        s.spawn(|_| {
            if let Err(err) = load_names_native(config) {
                eprintln!("Error loading japanese name suggestions {}", err);
            }
        });
    });
    Ok(())
}

// Load word suggestion index
fn load_words(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut index_map: HashMap<Language, BasicIndex> = HashMap::with_capacity(9);
    for language in Language::iter() {
        let path = Path::new(config.get_suggestion_sources()).join(format!("new_word_{language}"));
        if !path.exists() {
            log::warn!("Running without {language} suggestions");
            continue;
        }
        if language != Language::Japanese {
            let index: BasicIndex = bincode::deserialize_from(BufReader::new(File::open(path)?))?;
            index_map.insert(language, index);
        } else {
            let index: JapaneseIndex =
                bincode::deserialize_from(BufReader::new(File::open(path)?))?;
            JP_WORD_INDEX.set(index).ok().expect("JP Index alredy set");
        }
    }

    WORD_INDEX
        .set(index_map)
        .ok()
        .expect("WORD_INDEX already set!");

    Ok(())
}

/// Load kanji meaning suggestion file into memory
fn load_meanings(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("kanji_meanings");
    if !file.exists() {
        info!("Kanji-meaning suggestion file does not exists");
        return Ok(());
    }

    let index: JapaneseIndex = bincode::deserialize_from(BufReader::new(File::open(file)?))?;
    K_MEANING_SUGGESTIONS
        .set(index)
        .ok()
        .expect("won't happen lol");

    Ok(())
}

/// Load kanji meaning suggestion file into memory
fn load_names_foreign(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("names_trans");
    if !file.exists() {
        info!("Name transcription suggestion index not found");
        return Ok(());
    }

    let index: BasicIndex = bincode::deserialize_from(BufReader::new(File::open(file)?))?;
    NAME_TRANSCRIPTIONS
        .set(index)
        .ok()
        .expect("won't happen lol");

    Ok(())
}

/// Load kanji meaning suggestion file into memory
fn load_names_native(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("names_native");
    if !file.exists() {
        info!("Native japanese name index not found");
        return Ok(());
    }

    let index: JapaneseIndex = bincode::deserialize_from(BufReader::new(File::open(file)?))?;
    NAME_NATIVE.set(index).ok().expect("won't happen lol");

    Ok(())
}
