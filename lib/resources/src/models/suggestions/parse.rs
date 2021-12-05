use std::{error::Error, path::Path, str::FromStr};

use types::jotoba::languages::Language;

use crate::models::storage::{suggestion::SuggestionDictionary, SuggestionData};

pub(crate) fn load<P: AsRef<Path>>(
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
        return Ok(());
    }

    if let Some(lang_str) = file_name.strip_prefix("words_") {
        let lang = Language::from_str(lang_str)?;
        let dict = SuggestionDictionary::load(suggestion_file)?;
        suggestion_data.add_foreign(lang, dict);
        return Ok(());
    }

    Ok(())
}
