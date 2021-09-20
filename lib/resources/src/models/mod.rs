pub mod accents;
pub mod kanji;
pub mod names;
pub mod sentences;
pub mod storage;
pub mod suggestions;
pub mod words;

use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

use self::{
    kanji::Kanji,
    names::Name,
    storage::{RadicalStorage, ResourceStorage, SentenceStorage},
    words::Word,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DictResources {
    pub words: Vec<Word>,
    pub kanji: Vec<Kanji>,
    pub names: Vec<Name>,
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
    Ok(bincode::deserialize_from(File::open(sentences)?)?)
}