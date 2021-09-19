use japanese::{furigana, furigana::SentencePartRef};
use resources::parse::jmdict::languages::Language;

#[derive(Debug, PartialEq, Clone)]
pub struct Sentence {
    pub content: String,
    pub furigana: String,
    pub translation: String,
    pub language: Language,
    pub eng: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub sentence: Sentence,
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
        s: resources::models::sentences::Sentence,
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
