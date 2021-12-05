use japanese::{furigana, furigana::SentencePartRef};
use types::jotoba::languages::Language;

#[derive(PartialEq, Clone, Default)]
pub struct SentenceResult {
    pub items: Vec<Item>,
    pub len: usize,
    pub hidden: bool,
}

#[derive(PartialEq, Clone)]
pub struct Item {
    pub sentence: Sentence,
}

#[derive(PartialEq, Clone)]
pub struct Sentence {
    pub content: String,
    pub furigana: String,
    pub translation: String,
    pub language: Language,
    pub eng: String,
}

impl Sentence {
    #[inline]
    pub fn furigana_pairs<'a>(&'a self) -> impl Iterator<Item = SentencePartRef<'a>> {
        furigana::from_str(&self.furigana)
    }

    #[inline]
    pub fn get_english(&self) -> Option<&str> {
        if self.eng == "-" {
            None
        } else {
            Some(&self.eng)
        }
    }

    #[inline]
    pub fn from_m_sentence(
        s: types::jotoba::sentences::Sentence,
        language: Language,
        allow_english: bool,
    ) -> Option<Self> {
        let mut translation = s.get_translations(language);
        if translation.is_none() && allow_english {
            translation = s.get_translations(Language::English);
        }
        Some(Self {
            translation: translation?.to_string(),
            content: s.japanese,
            furigana: s.furigana,
            eng: String::from("-"),
            language,
        })
    }
}

impl From<(Vec<Item>, usize, bool)> for SentenceResult {
    fn from((items, len, hidden): (Vec<Item>, usize, bool)) -> Self {
        Self { items, len, hidden }
    }
}
