use std::hash::{Hash, Hasher};
use types::jotoba::languages::Language;

/// In-cookie saved personalized settings by an user
#[derive(Debug, Clone, Copy)]
pub struct UserSettings {
    pub user_lang: Language,
    pub page_lang: localization::language::Language,
    pub show_english: bool,
    pub english_on_top: bool,
    pub cookies_enabled: bool,
    pub page_size: u32,
    pub kanji_page_size: u32,
    pub show_example_sentences: bool,
    pub sentence_furigana: bool,
}

impl UserSettings {
    /// Returns `true` if an action has to be done for english too. This
    /// Is the case if the user wants to see enlgish results as well but
    /// didn't set english as main language
    #[inline]
    pub fn show_english(&self) -> bool {
        self.show_english && self.user_lang != Language::English
    }

    #[inline]
    pub fn language(&self) -> Language {
        self.user_lang
    }
}

impl Default for UserSettings {
    #[inline]
    fn default() -> Self {
        Self {
            show_english: true,
            user_lang: Language::default(),
            page_lang: localization::language::Language::default(),
            english_on_top: false,
            cookies_enabled: false,
            page_size: 10,
            kanji_page_size: 4,
            show_example_sentences: true,
            sentence_furigana: true,
        }
    }
}

impl PartialEq for UserSettings {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.user_lang == other.user_lang && self.show_english == other.show_english
    }
}

impl Hash for UserSettings {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user_lang.hash(state);
        self.show_english.hash(state);
    }
}
