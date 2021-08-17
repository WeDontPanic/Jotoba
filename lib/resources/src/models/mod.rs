pub mod accents;
pub mod kanji;
pub mod names;
pub mod sentences;
pub mod storage;
pub mod suggestions;
pub mod words;

use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    path::Path,
    str::FromStr,
};

use crate::{
    models::storage::suggestion::SuggestionDictionary, parse::jmdict::languages::Language,
};

use self::{
    kanji::Kanji,
    names::Name,
    storage::{ResourceStorage, SuggestionData},
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
) -> Result<ResourceStorage, Box<dyn Error>> {
    let dict_data = load_dict_data(dict_data_path)?;
    let suggestion_data = load_suggestions(suggestion_path)?;
    Ok(ResourceStorage::new(dict_data, suggestion_data))
}

fn load_dict_data<P: AsRef<Path>>(dict_data_path: P) -> Result<DictResources, Box<dyn Error>> {
    let data_reader = BufReader::new(File::open(dict_data_path)?);
    DictResources::read(data_reader)
}

fn load_suggestions<P: AsRef<Path>>(
    suggestion_path: P,
) -> Result<Option<SuggestionData>, Box<dyn Error>> {
    let suggestion_path = suggestion_path.as_ref();
    if !suggestion_path.exists() || !suggestion_path.is_dir() {
        return Ok(None);
    }

    // All items within the configured suggestion directory
    let dir_entries = std::fs::read_dir(suggestion_path).and_then(|i| {
        i.map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
    })?;

    let mut suggestion_data = SuggestionData::new();

    // Load each file and add to `suggestion_data`
    for entry in dir_entries {
        load_suggestion_file(entry, &mut suggestion_data)?;
    }

    Ok((!suggestion_data.is_empty()).then(|| suggestion_data))
}

fn load_suggestion_file<P: AsRef<Path>>(
    suggestion_file: P,
    suggestion_data: &mut SuggestionData,
) -> Result<(), Box<dyn Error>> {
    let file_name = suggestion_file
        .as_ref()
        .file_name()
        .and_then(|i| i.to_str().map(|i| i.to_owned()))
        .unwrap();

    if file_name == "words_ja-JP" {
        let dict = SuggestionDictionary::load(suggestion_file)?;
        suggestion_data.add_jp(dict);
        println!("loaded jp suggestions");
        return Ok(());
    }

    if let Some(lang_str) = file_name.strip_prefix("words_") {
        let lang = Language::from_str(lang_str)?;
        let dict = SuggestionDictionary::load(suggestion_file)?;
        suggestion_data.add_foreign(lang, dict);
        println!("loaded {} suggestions", lang);
        return Ok(());
    }

    Ok(())
}
