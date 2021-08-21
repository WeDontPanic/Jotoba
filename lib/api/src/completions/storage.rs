use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use config::Config;
use itertools::Itertools;
use log::info;
use once_cell::sync::OnceCell;
use search::suggestions::{store_item, TextSearch};

use super::WordPair;

/// In-memory storage for native name suggestions
pub(crate) static NAME_NATIVE: OnceCell<TextSearch<Vec<NameNative>>> = OnceCell::new();

/// In-memory storage for name transcriptions suggestions
pub(crate) static NAME_TRANSCRIPTIONS: OnceCell<TextSearch<Vec<NameTranscription>>> =
    OnceCell::new();

/// In-memory storage for kanji meaning suggestions
pub(crate) static K_MEANING_SUGGESTIONS: OnceCell<TextSearch<Vec<KanjiMeaningSuggestionItem>>> =
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

impl Into<WordPair> for &KanjiMeaningSuggestionItem {
    #[inline]
    fn into(self) -> WordPair {
        WordPair {
            primary: self.meaning.clone(),
            secondary: Some(self.literal.to_string()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SuggestionVersion {
    V0,
    V1,
}

trait Parseable: Sized {
    fn parse(s: &str, version: SuggestionVersion) -> Result<Self, error::Error>;
}

#[derive(Clone, Debug)]
pub struct NameTranscription {
    pub name: String,
    pub hash: eudex::Hash,
}

impl Parseable for NameTranscription {
    #[inline]
    fn parse(s: &str, _version: SuggestionVersion) -> Result<Self, error::Error> {
        Ok(NameTranscription {
            name: s.to_owned(),
            hash: eudex::Hash::new(s),
        })
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

#[derive(Clone, Debug)]
pub struct NameNative {
    pub name: String,
}

impl Parseable for NameNative {
    #[inline]
    fn parse(s: &str, _version: SuggestionVersion) -> Result<Self, error::Error> {
        Ok(NameNative { name: s.to_owned() })
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

pub fn load_suggestions(config: &Config) -> Result<(), Box<dyn Error>> {
    load_meaning_suggestions(&config)?;
    load_native_names(&config)?;
    load_name_transcriptions(&config)?;
    Ok(())
}

/// Load kanji meaning suggestion file into memory
fn load_meaning_suggestions(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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

/// Load native name suggestions
fn load_native_names(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = Path::new(config.get_suggestion_sources()).join("names_native");
    if !file.exists() {
        info!("Native name suggestion file does not exists");
        return Ok(());
    }

    let items: Vec<NameNative> = load_file(&file)?;

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

    let items: Vec<NameTranscription> = load_file(&file)?;

    NAME_TRANSCRIPTIONS.set(TextSearch::new(items)).ok();

    info!("Loaded name transcriptions suggestion file");

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
