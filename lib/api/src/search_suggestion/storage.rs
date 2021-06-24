use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};

use config::Config;
use itertools::Itertools;
use log::info;
use once_cell::sync::OnceCell;
use parse::jmdict::languages::Language;
use search::suggestions::{store_item, SuggestionSearch, TextSearch};

/// In-memory storage for kanji meaning suggestions
pub(crate) static K_MEANING_SUGGESTIONS: OnceCell<TextSearch<Vec<KanjiMeaningSuggestionItem>>> =
    OnceCell::new();

/// In-memory storage for wordsuggestions
pub(crate) static WORD_SUGGESTIONS: OnceCell<SuggestionSearch<Vec<WordSuggestionItem>>> =
    OnceCell::new();

/// A single suggestion item for kanji meanings
#[derive(Clone, Debug)]
pub struct KanjiMeaningSuggestionItem {
    pub meaning: String,
    pub literal: char,
    pub hash: eudex::Hash,
    pub score: i32,
}

impl store_item::Item for KanjiMeaningSuggestionItem {
    fn get_text(&self) -> &str {
        &self.meaning
    }

    fn get_hash(&self) -> eudex::Hash {
        self.hash
    }

    fn ord(&self) -> usize {
        self.score as usize
    }
}

impl Parseable for KanjiMeaningSuggestionItem {
    fn parse(s: &str, _version: SuggestionVersion) -> Result<Self, error::Error> {
        let mut split = s.split(',').rev();
        let score: i32 = split.next().ok_or(error::Error::ParseError)?.parse()?;
        let literal: char = split
            .next()
            .ok_or(error::Error::ParseError)?
            .chars()
            .next()
            .ok_or(error::Error::ParseError)?;
        let meaning: String = split.rev().join(",");

        Ok(KanjiMeaningSuggestionItem {
            // generate hash here so lookups will be faster
            hash: eudex::Hash::new(&meaning),
            meaning,
            literal,
            score,
        })
    }
}

/// A single suggestion item which exists of text and its sequence id to be able to assign results
/// to database entries
#[derive(Clone, Debug)]
pub struct WordSuggestionItem {
    pub text: String,
    pub sequence: i32,
    pub occurences: usize,
    pub hash: eudex::Hash,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SuggestionVersion {
    V0,
    V1,
}

impl store_item::Item for WordSuggestionItem {
    fn get_text(&self) -> &str {
        &self.text
    }

    fn get_hash(&self) -> eudex::Hash {
        self.hash
    }

    fn ord(&self) -> usize {
        self.occurences
    }
}

trait Parseable: Sized {
    fn parse(s: &str, version: SuggestionVersion) -> Result<Self, error::Error>;
}

impl Parseable for WordSuggestionItem {
    fn parse(s: &str, version: SuggestionVersion) -> Result<Self, error::Error> {
        let mut split = s.split(',').rev();
        let occurences = if version == SuggestionVersion::V1 {
            split.next().ok_or(error::Error::ParseError)?.parse()?
        } else {
            0
        };

        let number: i32 = split.next().ok_or(error::Error::ParseError)?.parse()?;

        let text: String = split.rev().join(",");

        Ok(WordSuggestionItem {
            // generate hash here so lookups will be faster
            hash: eudex::Hash::new(&text),
            text,
            occurences,
            sequence: number,
        })
    }
}

/// Load Suggestions from suggestion folder into memory
pub fn load_word_suggestions(config: &Config) -> Result<(), Box<dyn Error>> {
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
                if entry_name != "kanji_meanings" {
                    info!("Ignoring invalid suggestion-file {}", entry_name);
                }
                continue;
            }
        };

        map.insert(lang, TextSearch::new(load_file(&entry)?));
        info!("Loaded {:?} suggestion file", lang);
    }

    WORD_SUGGESTIONS.set(SuggestionSearch::new(map)).ok();
    Ok(())
}

/// Load kanji meaning suggestion file into memory
pub fn load_meaning_suggestions(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("kanji_meanings");
    if !file.exists() {
        info!("Kanji-meaning suggestion file does not exists");
        return Ok(());
    }

    let items: Vec<KanjiMeaningSuggestionItem> = load_file(&file)?;

    K_MEANING_SUGGESTIONS.set(TextSearch::new(items)).ok();

    info!("Loaded kanji meaning suggestion file");

    Ok(())
}

/// Parse a single suggestion file
fn load_file<T: Parseable>(path: &PathBuf) -> Result<Vec<T>, error::Error> {
    let file = File::open(path)?;

    let mut lines = BufReader::new(file).lines();

    let first = lines.next().ok_or(error::Error::ParseError)??;

    let version = match first.as_str() {
        "v1" => SuggestionVersion::V1,
        _ => SuggestionVersion::V0,
    };

    lines
        .map(|i| {
            i.map_err(|i| i.into())
                .and_then(move |i| T::parse(&i, version))
        })
        .collect::<Result<_, _>>()
}
