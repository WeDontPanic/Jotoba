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
    storage::{RadicalStorage, ResourceStorage},
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
    pub fn build<W: Write>(&self, out: &mut W) -> Result<(), serde_json::Error> {
        serde_json::to_writer(out, self)
    }

    /// Builds a new `ResourceStorage` from a reader containing json encoded data of all resources.
    /// This file can be create by `build`
    #[inline]
    pub fn read<R: Read>(reader: R) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_reader(reader)?)
    }
}

/// Load a resource storage from a BufReader
pub fn load_storage<P: AsRef<Path>>(
    dict_data_path: P,
    suggestion_path: P,
    rad_mapc_path: P,
) -> Result<ResourceStorage, Box<dyn Error>> {
    let dict_data = load_dict_data(dict_data_path)?;
    let suggestion_data = suggestions::parse::load(suggestion_path)?;
    let radical_map = load_rad_map(rad_mapc_path)?;
    Ok(ResourceStorage::new(
        dict_data,
        suggestion_data,
        radical_map,
    ))
}

fn load_dict_data<P: AsRef<Path>>(dict_data_path: P) -> Result<DictResources, Box<dyn Error>> {
    DictResources::read(BufReader::new(File::open(dict_data_path)?))
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
