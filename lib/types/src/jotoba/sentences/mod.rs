pub mod tag;

pub use self::tag::Tag;

use super::languages::Language;
use bitflags::BitFlag;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

/// A single Sentence with multiple translations.
#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Sentence {
    pub id: u32,
    pub japanese: String,
    pub furigana: String,
    pub translations: Vec<Translation>,
    pub jlpt_guess: Option<u8>,
    pub level: Option<i8>,
    pub tags: Vec<Tag>,
}

impl std::fmt::Debug for Sentence {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.japanese)
    }
}

/// A Translation for a sentence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub text: String,
    pub language: Language,
}

impl Sentence {
    /// Create a new sentence
    #[inline]
    pub fn new(
        id: u32,
        japanese: String,
        furigana: String,
        translations: Vec<Translation>,
        tags: Vec<Tag>,
    ) -> Self {
        Sentence {
            id,
            japanese,
            furigana,
            translations,
            jlpt_guess: None,
            level: None,
            tags,
        }
    }

    pub fn set_jlpt_guess(&mut self, guess: u8) {
        self.jlpt_guess = Some(guess)
    }

    /// Returns the kana reading of a sentence
    #[inline]
    #[cfg(feature = "jotoba_intern")]
    pub fn get_kana(&self) -> String {
        japanese::furigana::parse::from_str(&self.furigana)
            .map(|i| i.kana)
            .collect()
    }

    #[cfg(feature = "jotoba_intern")]
    pub fn get_furigana(&self) -> impl Iterator<Item = japanese::furigana::SentencePartRef> {
        japanese::furigana::parse::from_str(&self.furigana)
    }

    /// Returns `true` if the sentence has the given tag
    #[inline]
    pub fn has_tag(&self, tag: &Tag) -> bool {
        self.tags.iter().any(|i| i == tag)
    }

    /// Returns `true` if the sentence contains a translation for `language`
    #[inline]
    pub fn has_translation(&self, language: Language) -> bool {
        self.translations.iter().any(|tr| tr.language == language)
    }

    /// Returns the translation for a given language if exists
    #[inline]
    pub fn translation_for(&self, language: Language) -> Option<&str> {
        self.translations
            .iter()
            .find(|i| i.language == language)
            .map(|i| i.text.as_str())
    }

    pub fn get_translation(&self, language: Language, allow_english: bool) -> Option<&str> {
        if let Some(s) = self.translation_for(language) {
            return Some(s);
        }

        if allow_english {
            return self.translation_for(Language::English);
        }

        None
    }

    /// Calculates a bitmask to efficiently determine the supported languages of a sentence
    pub fn calc_lang_mask(&self) -> u16 {
        lang_mask(self.translations.iter().map(|i| i.language))
    }
}

pub fn lang_mask<I>(langs: I) -> u16
where
    I: Iterator<Item = Language>,
{
    let mut lang_mask = BitFlag::<u16>::new();
    for lang in langs {
        let lang: i32 = lang.into();
        lang_mask.set_unchecked(lang as u16, true);
    }
    lang_mask.raw()
}

pub fn parse_lang_mask(mask: u16) -> Vec<Language> {
    let mut langs = Vec::new();
    for i in 0..10 {
        if mask & (1 << i) == 0 {
            continue;
        }
        if let Ok(lang) = Language::try_from(i as i32) {
            langs.push(lang);
        }
    }
    langs
}

impl From<(String, Language)> for Translation {
    #[inline]
    fn from((text, language): (String, Language)) -> Self {
        Self { text, language }
    }
}

impl PartialEq for Sentence {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Sentence {}

impl Hash for Sentence {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
