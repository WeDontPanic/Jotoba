use super::utils;
use crate::{
    kanji_reading,
    regex::RegexSearchIndex,
    relevance::RelevanceIndex,
    words::{ForeignIndex, NativeIndex},
};
use std::{collections::HashMap, error::Error, path::Path};
use types::jotoba::languages::Language;

const FOREIGN_PREFIX: &str = "word_index";
const NATIVE_FILE: &str = "jp_index";
const REGEX_FILE: &str = "regex_index";
const RELEVANCE_PREFIX: &str = "relevance_index_";
const KANJI_READING_INDEX: &str = "word_kr_index";

/// Store for words
pub struct WordStore {
    foreign: HashMap<Language, ForeignIndex>,
    native: NativeIndex,

    regex: RegexSearchIndex,
    relevance: HashMap<Language, RelevanceIndex>,

    k_reading: kanji_reading::Index,
}

impl WordStore {
    pub(crate) fn new(
        foreign: HashMap<Language, ForeignIndex>,
        native: NativeIndex,
        regex: RegexSearchIndex,
        relevance: HashMap<Language, RelevanceIndex>,
        k_reading: kanji_reading::Index,
    ) -> Self {
        Self {
            foreign,
            native,
            regex,
            relevance,
            k_reading,
        }
    }

    /// Returns the foreign index for the given language
    #[inline]
    pub fn foreign(&self, language: Language) -> Option<&ForeignIndex> {
        self.foreign.get(&language)
    }

    #[inline]
    pub fn native(&self) -> &NativeIndex {
        &self.native
    }

    #[inline]
    pub fn regex(&self) -> &RegexSearchIndex {
        &self.regex
    }

    #[inline]
    pub fn relevance(&self, language: Language) -> Option<&RelevanceIndex> {
        self.relevance.get(&language)
    }

    pub(crate) fn check(&self) -> bool {
        utils::check_lang_map(&self.foreign)
    }

    pub fn k_reading(&self) -> &kanji_reading::Index {
        &self.k_reading
    }
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<WordStore, Box<dyn Error>> {
    let foreign = load_foreign(path.as_ref())?;
    let native = NativeIndex::open(path.as_ref().join(NATIVE_FILE))?;
    let regex = utils::deser_file(path.as_ref(), REGEX_FILE)?;
    let relevance = load_rel_index(path.as_ref())?;
    let k_reading = kanji_reading::Index::open(path.as_ref().join(KANJI_READING_INDEX))?;
    Ok(WordStore::new(foreign, native, regex, relevance, k_reading))
}

fn load_foreign<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<Language, ForeignIndex>, Box<dyn Error>> {
    utils::load_by_language(path, FOREIGN_PREFIX, |p| {
        let index = ForeignIndex::open(p)?;
        let lang = index.get_metadata().language;
        Ok(Some((lang, index)))
    })
}

fn load_rel_index<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<Language, RelevanceIndex>, Box<dyn Error>> {
    utils::load_by_language(path, RELEVANCE_PREFIX, |p| {
        let lang = match utils::lang_from_file(p, RELEVANCE_PREFIX) {
            Some(l) => l,
            None => return Ok(None),
        };
        let rel_index: RelevanceIndex = utils::deser_file(p, "")?;
        Ok(Some((lang, rel_index)))
    })
}
