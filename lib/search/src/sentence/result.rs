use japanese::{furigana, furigana::SentencePartRef};
use types::jotoba::language::{param::AsLangParam, Language};

use crate::executor::out_builder::OutputAddable;

/// Additional result data for a sentence search
#[derive(Clone, Copy, Default, Debug)]
pub struct ResData {
    pub hidden: bool,
}

impl ResData {
    pub fn new(hidden: bool) -> Self {
        Self { hidden }
    }
}

impl OutputAddable for ResData {}

/// A displayable sentence
#[derive(Clone, Debug)]
pub struct Sentence {
    pub id: u32,
    pub content: &'static str,
    pub furigana: &'static str,
    pub translation: &'static str,
    pub language: Language,
    pub eng: Option<String>,
}

impl Sentence {
    #[inline]
    pub fn furigana_pairs<'a>(&'a self) -> impl Iterator<Item = SentencePartRef<'a>> {
        furigana::parse::from_str(&self.furigana)
    }

    #[inline]
    pub fn get_english(&self) -> Option<&str> {
        self.eng.as_deref()
    }

    #[inline]
    pub fn from_m_sentence(
        s: &'static types::jotoba::sentences::Sentence,
        lang: impl AsLangParam,
    ) -> Option<Self> {
        let translation = s.get_translation(lang)?;

        Some(Self {
            id: s.id,
            translation,
            content: &s.japanese,
            furigana: &s.furigana,
            eng: None,
            language: lang.as_lang().language(),
        })
    }
}
