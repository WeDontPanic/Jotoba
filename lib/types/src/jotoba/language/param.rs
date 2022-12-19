use super::Language;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Language parameter that contains a Language and whether English should be used as fallback
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct LangParam {
    lang: Language,
    use_en: bool,
}

impl LangParam {
    /// Creates a new LangParam with English fallback disabled
    #[inline]
    pub fn new(lang: Language) -> Self {
        Self::with_en_raw(lang, false)
    }

    /// Creates a new LangParam with English fallback enabled
    #[inline]
    pub fn with_en(lang: Language) -> Self {
        Self::with_en_raw(lang, true)
    }

    /// Creates a new LangParam with English fallback as custom parameter
    #[inline]
    pub fn with_en_raw(lang: Language, use_en: bool) -> Self {
        Self { lang, use_en }
    }

    /// Returns `true` whether English can be used
    #[inline]
    pub fn en_fallback(&self) -> bool {
        self.use_en
    }

    /// Returns `true` if the language is `Language::English`
    #[inline]
    pub fn is_english(&self) -> bool {
        self.lang == Language::English
    }

    /// Returns the params language
    #[inline]
    pub fn language(&self) -> Language {
        self.lang
    }

    /// Returns `true` if the language param matches the given language. This also uses `use_en`
    /// for the comparison
    #[inline]
    pub fn eq_to_lang(&self, lang: &Language) -> bool {
        self.lang == *lang || (self.en_fallback() && *lang == Language::English)
    }
}

impl Deref for LangParam {
    type Target = Language;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lang
    }
}

// Little shortcut to make trait bounds easier to read
pub trait AsLangParam: Copy {
    fn as_lang(self) -> LangParam;
}

impl<T: Into<LangParam> + Copy> AsLangParam for T {
    #[inline]
    fn as_lang(self) -> LangParam {
        self.into()
    }
}

impl From<&Language> for LangParam {
    #[inline]
    fn from(lang: &Language) -> Self {
        Self::new(*lang)
    }
}

impl From<Language> for LangParam {
    #[inline]
    fn from(lang: Language) -> Self {
        Self::new(lang)
    }
}

impl From<(Language, bool)> for LangParam {
    #[inline]
    fn from(lang: (Language, bool)) -> Self {
        Self::with_en_raw(lang.0, lang.1)
    }
}
