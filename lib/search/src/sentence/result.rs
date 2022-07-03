use japanese::{furigana, furigana::SentencePartRef};
use types::jotoba::languages::Language;

#[derive(Clone, Default)]
pub struct SentenceResult {
    pub items: Vec<Sentence>,
    pub len: usize,
    pub hidden: bool,
}

#[derive(Clone)]
pub struct Sentence {
    pub id: u32,
    pub content: String,
    pub furigana: String,
    pub translation: String,
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
        s: types::jotoba::sentences::Sentence,
        language: Language,
        allow_english: bool,
    ) -> Option<Self> {
        let mut translation = s
            .translation_for(language)
            .or_else(|| s.translation_for(Language::English));
        if translation.is_none() && allow_english {
            translation = s.translation_for(Language::English);
        }

        Some(Self {
            id: s.id,
            translation: translation?.to_string(),
            content: s.japanese,
            furigana: s.furigana,
            eng: None,
            language,
        })
    }
}

impl From<(Vec<Sentence>, usize, bool)> for SentenceResult {
    #[inline]
    fn from((items, len, hidden): (Vec<Sentence>, usize, bool)) -> Self {
        Self { items, len, hidden }
    }
}
