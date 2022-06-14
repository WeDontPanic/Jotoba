use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    str::FromStr,
};
use types::jotoba::languages::Language;

/// Deserializes a file from `path` with `name`
pub fn deser_file<O: DeserializeOwned, P: AsRef<Path>>(
    path: P,
    name: &str,
) -> Result<O, Box<dyn Error + Send + Sync>> {
    let path = if name.is_empty() {
        path.as_ref().to_path_buf()
    } else {
        path.as_ref().join(name)
    };
    Ok(fast_deser(path)?)
}

pub fn load_by_language<O, F, P: AsRef<Path>>(
    path: P,
    prefix: &str,
    load: F,
) -> Result<HashMap<Language, O>, Box<dyn Error + Send + Sync>>
where
    F: Fn(&Path) -> Result<Option<(Language, O)>, Box<dyn Error + Sync + Send>>,
{
    let mut map = HashMap::with_capacity(10);

    // All index files in index source folder
    let files = std::fs::read_dir(path)?.map(|res| res.map(|e| e.path()));

    for file in files {
        let file = file?;

        let file_name = file.file_name().and_then(|i| i.to_str()).unwrap();
        if !file_name.starts_with(prefix) {
            continue;
        }

        match load(file.as_ref())? {
            Some((lang, deser)) => {
                map.insert(lang, deser);
            }
            None => (),
        };
    }

    Ok(map)
}

pub fn lang_from_file<F: AsRef<Path>>(file: F, prefix: &str) -> Option<Language> {
    let file_name = file.as_ref().file_name()?.to_str()?.to_string();
    let lang_str = file_name.strip_prefix(prefix).unwrap();
    Language::from_str(lang_str).ok()
}

/// Returns true if `map` has an entry for all language keys
pub fn check_lang_map<T>(map: &HashMap<Language, T>) -> bool {
    Language::iter_word().all(|i| map.contains_key(&i))
}

// A bit faster. Who cares about memory consumption anyways
fn fast_deser<O: DeserializeOwned, P: AsRef<Path>>(
    file_path: P,
) -> Result<O, Box<dyn Error + Sync + Send>> {
    let file = File::open(file_path)?;
    let len = file.metadata()?.len();
    let mut buf = vec![0u8; len as usize];
    let mut reader = BufReader::new(file);
    reader.read_exact(&mut buf)?;
    Ok(bincode::deserialize(&buf)?)
}
