pub mod storage;
pub mod suggestions;

use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

use self::storage::{RadicalStorage, ResourceStorage, SentenceStorage};
use serde::{Deserialize, Serialize};
use types::jotoba::{
    kanji::{DetailedRadical, Kanji},
    names::Name,
    words::Word,
};

/// Static git hash of current build
pub const GIT_HASH: &str = env!("GIT_HASH");

#[derive(Debug, Serialize, Deserialize)]
pub struct DictResources {
    // words
    pub words: Vec<Word>,
    pub word_jlpt: HashMap<u8, Vec<u32>>,
    pub irregular_iru_eru: Vec<u32>,
    // kanji
    pub kanji: Vec<Kanji>,
    pub kanji_genki: HashMap<u8, Vec<char>>,
    pub kanji_jlpt: HashMap<u8, Vec<char>>,
    // names
    pub names: Vec<Name>,
    // radicals
    pub radicals: Vec<DetailedRadical>,
}

impl DictResources {
    /// Writes the resource storage into `out`
    #[inline]
    pub fn build<W: Write>(&self, out: &mut W) -> Result<(), bincode::Error> {
        bincode::serialize_into(out, &self)
    }

    /// Builds a new `ResourceStorage` from a reader containing encoded data of all resources.
    /// This file can be create by `build`
    #[inline]
    pub fn read<R: Read>(reader: R) -> Result<Self, bincode::Error> {
        bincode::deserialize_from(reader)
    }
}

/// Load a resource storage from a BufReader
pub fn load_storage<P: AsRef<Path>>(
    dict_data_path: P,
    suggestion_path: P,
    rad_mapc_path: P,
    sentences_path: P,
) -> Result<ResourceStorage, Box<dyn Error>> {
    let dict_data = load_dict_data(dict_data_path)?;
    let suggestion_data = suggestions::parse::load(suggestion_path)?;
    let radical_map = load_rad_map(rad_mapc_path)?;
    let sentences = load_sentences(sentences_path)?;

    Ok(ResourceStorage::new(
        dict_data,
        suggestion_data,
        radical_map,
        sentences,
    ))
}

#[inline]
pub fn load_dict_data<P: AsRef<Path>>(dict_data_path: P) -> Result<DictResources, Box<dyn Error>> {
    Ok(DictResources::read(BufReader::new(File::open(
        dict_data_path,
    )?))?)
}

fn load_rad_map<P: AsRef<Path>>(rad_map_file: P) -> Result<RadicalStorage, Box<dyn Error>> {
    let reader = BufReader::new(File::open(rad_map_file)?);
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        if line.chars().count() < 2 {
            continue;
        }
        let mut chars = line.chars();
        let rad_literal = chars.next().unwrap();
        let kanji_literals = chars.collect::<Vec<_>>();
        map.insert(rad_literal, kanji_literals);
    }

    Ok(map)
}

/// Load sentences from sentence file
fn load_sentences<P: AsRef<Path>>(sentences: P) -> Result<SentenceStorage, Box<dyn Error>> {
    Ok(bincode::deserialize_from(BufReader::new(File::open(
        sentences,
    )?))?)
}
