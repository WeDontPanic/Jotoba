use super::utils;
use autocompletion::index::{basic::BasicIndex, japanese::JapaneseIndex};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, path::Path};
use types::jotoba::languages::Language;

/// In-memory store for all suggestion indexes
pub(crate) static SUGGESTION_STORE: OnceCell<SuggestionStorage> = OnceCell::new();

/// Contains all suggestion index data
#[derive(Serialize, Deserialize)]
pub struct SuggestionStorage {
    jp_words: JapaneseIndex,
    foreign_words: HashMap<Language, BasicIndex>,

    kanji_meanings: JapaneseIndex,

    names_native: JapaneseIndex,
    names_foreign: BasicIndex,
}

impl SuggestionStorage {
    pub fn new(
        jp_words: JapaneseIndex,
        foreign_words: HashMap<Language, BasicIndex>,
        kanji_meanings: JapaneseIndex,
        names_native: JapaneseIndex,
        names_foreign: BasicIndex,
    ) -> Self {
        Self {
            jp_words,
            foreign_words,
            kanji_meanings,
            names_native,
            names_foreign,
        }
    }

    #[inline]
    pub fn jp_words(&self) -> &JapaneseIndex {
        &self.jp_words
    }

    #[inline]
    pub fn foreign_words(&self, language: Language) -> Option<&BasicIndex> {
        self.foreign_words.get(&language)
    }

    #[inline]
    pub fn kanji_meanings(&self) -> &JapaneseIndex {
        &self.kanji_meanings
    }

    #[inline]
    pub fn names_native(&self) -> &JapaneseIndex {
        &self.names_native
    }

    #[inline]
    pub fn names_foreign(&self) -> &BasicIndex {
        &self.names_foreign
    }

    pub fn check(&self) -> bool {
        utils::check_lang_map(&self.foreign_words)
    }
}

pub fn load_raw<P: AsRef<Path>>(
    file: P,
) -> Result<SuggestionStorage, Box<dyn Error + Send + Sync>> {
    utils::deser_file(file, "")
}

pub fn load<P: AsRef<Path>>(file: P) -> Result<bool, Box<dyn Error + Sync + Send>> {
    let store = load_raw(file)?;
    Ok(SUGGESTION_STORE.set(store).is_ok())
}

#[inline]
pub fn get_suggestions() -> &'static SuggestionStorage {
    unsafe { SUGGESTION_STORE.get_unchecked() }
}
