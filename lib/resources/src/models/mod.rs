pub mod accents;
pub mod kanji;
pub mod names;
pub mod sentences;
pub mod storage;
pub mod suggestions;
pub mod words;

use std::io::{BufReader, Read, Write};

use self::{kanji::Kanji, names::Name, storage::ResourceStorage, words::Word};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resources {
    pub words: Vec<Word>,
    pub kanji: Vec<Kanji>,
    pub names: Vec<Name>,
}

impl Resources {
    /// Writes the resource storage into `out`
    #[inline]
    pub fn build<W: Write>(self, out: &mut W) -> Result<(), std::io::Error> {
        let out_text = serde_json::to_string(&self)?;
        out.write_all(out_text.as_bytes())?;
        Ok(())
    }

    /// Builds a new `ResourceStorage` from a reader containing json encoded data of all resources.
    /// This file can be create by `build`
    #[inline]
    pub fn read<R: Read>(reader: R) -> Result<Self, std::io::Error> {
        let out = serde_json::from_reader(reader)?;
        Ok(out)
    }
}

/// Load a resource storage from a BufReader
pub fn load_stoarge<R: Read>(reader: BufReader<R>) -> Result<ResourceStorage, std::io::Error> {
    Ok(ResourceStorage::new(Resources::read(reader)?))
}
