use super::utils;
use crate::{
    regex::RegexSearchIndex,
    relevance::RelevanceIndex,
    words::{ForeignIndex, NativeIndex},
};
use std::{collections::HashMap, error::Error, path::Path};
use types::jotoba::languages::Language;

const REGEX_FILE: &str = "regex_index";
const FOREIGN_PREFIX: &str = "word_index";
const NATIVE_FILE: &str = "jp_index";
const RELEVANCE_PREFIX: &str = "relevance_index_";

/// Store for names
pub struct WordStore {
    foreign: HashMap<Language, ForeignIndex>,
    native: NativeIndex,
    regex: RegexSearchIndex,

    relevance: HashMap<Language, RelevanceIndex>,
}

impl WordStore {
    pub(crate) fn new(
        foreign: HashMap<Language, ForeignIndex>,
        native: NativeIndex,
        regex: RegexSearchIndex,
        relevance: HashMap<Language, RelevanceIndex>,
    ) -> Self {
        Self {
            foreign,
            native,
            regex,
            relevance,
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
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<WordStore, Box<dyn Error>> {
    let foreign = load_foreign(path.as_ref())?;
    let native = NativeIndex::open(path.as_ref().join(NATIVE_FILE))?;
    let regex = utils::deser_file(path.as_ref(), REGEX_FILE)?;
    let relevance = load_rel_index(path.as_ref())?;
    Ok(WordStore::new(foreign, native, regex, relevance))
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
