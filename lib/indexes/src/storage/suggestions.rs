use crate::hashtag::HashTagIndex;

use super::utils;
use autocompletion::index::{basic::BasicIndex, japanese::JapaneseIndex};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, path::Path};
use types::jotoba::language::Language;

pub const K_MEANING_NGRAM: usize = 3;

pub const FG_WORDS_NGRAM: usize = 3;
pub const JP_WORDS_NGRAM: usize = 2;

pub const FG_NAMES_NGRAM: usize = 3;
pub const JP_NAMES_NGRAM: usize = 2;

pub const SUGGESTION_FILE: &str = "suggestions";

/// In-memory store for all suggestion indexes
pub(crate) static SUGGESTION_STORE: OnceCell<SuggestionStorage> = OnceCell::new();

/// Contains all suggestion index data
#[derive(Serialize, Deserialize)]
pub struct SuggestionStorage {
    jp_words: JapaneseIndex<JP_WORDS_NGRAM>,
    foreign_words: HashMap<Language, BasicIndex<FG_WORDS_NGRAM>>,

    kanji_meanings: JapaneseIndex<K_MEANING_NGRAM>,

    names_native: JapaneseIndex<JP_NAMES_NGRAM>,
    names_foreign: BasicIndex<FG_NAMES_NGRAM>,

    hashtag: HashTagIndex,
}

impl SuggestionStorage {
    pub fn new(
        jp_words: JapaneseIndex,
        foreign_words: HashMap<Language, BasicIndex<FG_WORDS_NGRAM>>,
        kanji_meanings: JapaneseIndex<K_MEANING_NGRAM>,
        names_native: JapaneseIndex<JP_NAMES_NGRAM>,
        names_foreign: BasicIndex<FG_NAMES_NGRAM>,
        hashtag: HashTagIndex,
    ) -> Self {
        Self {
            jp_words,
            foreign_words,
            kanji_meanings,
            names_native,
            names_foreign,
            hashtag,
        }
    }

    #[inline]
    pub fn jp_words(&self) -> &JapaneseIndex {
        &self.jp_words
    }

    #[inline]
    pub fn foreign_words(&self, language: Language) -> Option<&BasicIndex<FG_WORDS_NGRAM>> {
        self.foreign_words.get(&language)
    }

    #[inline]
    pub fn kanji_meanings(&self) -> &JapaneseIndex<K_MEANING_NGRAM> {
        &self.kanji_meanings
    }

    #[inline]
    pub fn names_native(&self) -> &JapaneseIndex<JP_NAMES_NGRAM> {
        &self.names_native
    }

    #[inline]
    pub fn names_foreign(&self) -> &BasicIndex<FG_NAMES_NGRAM> {
        &self.names_foreign
    }

    #[inline]
    pub fn hashtags(&self) -> &HashTagIndex {
        &self.hashtag
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

pub fn load<P: AsRef<Path>>(res_dir: P) -> Result<bool, Box<dyn Error + Sync + Send>> {
    let store = load_raw(res_dir.as_ref().join(SUGGESTION_FILE))?;
    Ok(SUGGESTION_STORE.set(store).is_ok())
}

#[inline]
pub fn get_suggestions() -> &'static SuggestionStorage {
    unsafe { SUGGESTION_STORE.get_unchecked() }
}
