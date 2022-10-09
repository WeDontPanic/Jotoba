use super::utils;
use crate::{
    kanji,
    regex::RegexSearchIndex,
    words::{ForeignIndex, NativeIndex},
};
use log::debug;
use std::{collections::HashMap, error::Error, path::Path, str::FromStr};
use types::jotoba::languages::Language;

const FOREIGN_PREFIX: &str = "word_index_";
const NATIVE_FILE: &str = "jp_index";
const REGEX_FILE: &str = "regex_index";
const KANJI_READING_INDEX: &str = "word_kr_index";

/// Store for words
pub struct WordStore {
    foreign: HashMap<Language, ForeignIndex>,
    native: NativeIndex,

    regex: RegexSearchIndex,

    k_reading: kanji::reading::Index,
}

impl WordStore {
    pub(crate) fn new(
        foreign: HashMap<Language, ForeignIndex>,
        native: NativeIndex,
        regex: RegexSearchIndex,
        k_reading: kanji::reading::Index,
    ) -> Self {
        Self {
            foreign,
            native,
            regex,
            k_reading,
        }
    }

    /// Returns the foreign index for the given language
    #[inline]
    pub fn foreign(&self, language: Language) -> Option<&ForeignIndex> {
        self.foreign.get(&language)
    }

    #[inline]
    pub fn regex(&self) -> &RegexSearchIndex {
        &self.regex
    }

    #[inline]
    pub fn k_reading(&self) -> &kanji::reading::Index {
        &self.k_reading
    }

    #[inline]
    pub fn native(&self) -> &NativeIndex {
        &self.native
    }

    pub(crate) fn check(&self) -> bool {
        utils::check_lang_map(&self.foreign)
    }
}

#[cfg(not(feature = "parallel"))]
pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<WordStore, Box<dyn Error + Sync + Send>> {
    let start = std::time::Instant::now();
    let foreign = load_foreign(path.as_ref())?;
    let native = utils::deser_file(path.as_ref(), NATIVE_FILE)?;
    let regex = utils::deser_file(path.as_ref(), REGEX_FILE)?;
    let k_reading = utils::deser_file(path.as_ref(), KANJI_READING_INDEX)?;
    debug!("Loading indexes sync took: {:?}", start.elapsed());
    Ok(WordStore::new(foreign, native, regex, k_reading))
}

#[cfg(feature = "parallel")]
pub(crate) fn load<P: AsRef<Path> + Send + Sync>(
    path: P,
) -> Result<WordStore, Box<dyn Error + Send + Sync>> {
    let start = std::time::Instant::now();
    let mut foreign = None;
    let mut native = None;
    let mut regex: Option<Result<RegexSearchIndex, Box<dyn Error + Send + Sync>>> = None;
    let mut k_reading = None;
    rayon::scope(|s| {
        s.spawn(|_| {
            foreign = Some(load_foreign(path.as_ref()));
        });
        s.spawn(|_| {
            native = Some(utils::deser_file(path.as_ref(), NATIVE_FILE));
        });
        s.spawn(|_| {
            regex = Some(utils::deser_file(path.as_ref(), REGEX_FILE));
        });
        s.spawn(|_| {
            k_reading = Some(utils::deser_file(path.as_ref(), KANJI_READING_INDEX));
        });
    });
    let foreign = foreign.unwrap()?;
    let native = native.unwrap()?;
    let regex = regex.unwrap()?;
    let k_reading = k_reading.unwrap()?;
    debug!("Loading indexes parallel took: {:?}", start.elapsed());
    Ok(WordStore::new(foreign, native, regex, k_reading))
}

fn load_foreign<P: AsRef<Path>>(
    path: P,
) -> Result<HashMap<Language, ForeignIndex>, Box<dyn Error + Send + Sync>> {
    utils::load_by_language(path, FOREIGN_PREFIX, |p| {
        //let index = ForeignIndex::open(p)?;
        let index: ForeignIndex = utils::deser_file(p, "").unwrap();
        let file_name = p
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_prefix(FOREIGN_PREFIX)
            .unwrap();
        let lang = Language::from_str(file_name).unwrap();
        //let lang = index.get_metadata().language;
        Ok(Some((lang, index)))
    })
}
