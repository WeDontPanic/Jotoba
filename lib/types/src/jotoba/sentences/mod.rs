use bitflags::BitFlag;
use serde::{Deserialize, Serialize};

use super::languages::Language;

/// A single Sentence with multiple translations.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Default)]
pub struct Sentence {
    pub id: u32,
    pub japanese: String,
    pub furigana: String,
    pub translations: Vec<Translation>,
    pub jlpt_guess: Option<u8>,
    pub level: Option<i8>,
}

/// A Translation for a sentence
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    ) -> Self {
        Sentence {
            id,
            japanese,
            furigana,
            translations,
            jlpt_guess: None,
            level: None,
        }
    }

    pub fn set_jlpt_guess(&mut self, guess: u8) {
        self.jlpt_guess = Some(guess)
    }

    /// Returns the kana reading of a sentence
    #[inline]
    #[cfg(feature = "jotoba_intern")]
    pub fn get_kana(&self) -> String {
        japanese::furigana::from_str(&self.furigana)
            .map(|i| i.kana)
            .collect::<String>()
    }

    /// Returns `true` if the sentence contains a translation for `language`
    #[inline]
    pub fn has_translation(&self, language: Language) -> bool {
        self.translations.iter().any(|tr| tr.language == language)
    }

    /// Returns the translation for a given language if exists
    #[inline]
    pub fn get_translations(&self, language: Language) -> Option<&str> {
        self.translations
            .iter()
            .find(|i| i.language == language)
            .map(|i| i.text.as_str())
    }

    /// Calculates a bitmask to efficiently determine the supported languages of a sentence
    pub fn calc_lang_mask(&self) -> u16 {
        lang_mask(self.translations.iter().map(|i| i.language))
    }
}

pub fn lang_mask<I: Iterator<Item = Language>>(langs: I) -> u16 {
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
