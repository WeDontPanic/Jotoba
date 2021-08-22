use std::{fs::read_to_string, path::Path, vec};

use resources::{
    models::{kanji::Kanji, words::Word},
    parse::jmdict::languages::Language,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub kanji: Kanji,
    pub kun_dicts: Option<Vec<Word>>,
    pub on_dicts: Option<Vec<Word>>,
}

impl Item {
    #[inline]
    pub fn from_db(k: Kanji, lang: Language, show_english: bool) -> Self {
        // TODO: Load on & kun dictionaries here
        Self {
            kun_dicts: None,
            on_dicts: None,
            kanji: k,
        }
    }
}

impl Item {
    /// Return the animation entries for the template
    pub fn get_animation_entries(&self) -> Vec<(String, String)> {
        if let Ok(content) = read_to_string(self.kanji.get_animation_path()) {
            content
                .split('\n')
                .into_iter()
                .map(|i| {
                    let mut s = i.split(';');
                    (s.next().unwrap().to_owned(), s.next().unwrap().to_owned())
                })
                .collect::<Vec<(String, String)>>()
        } else {
            vec![]
        }
    }

    /// Get a list of korean readings, formatted as: "<Hangul> (<romanized>)"
    pub fn get_korean(&self) -> Option<Vec<String>> {
        if self.kanji.korean_r.is_some() && self.kanji.korean_h.is_some() {
            let korean_h = self.kanji.korean_h.as_ref().unwrap();
            let korean_r = self.kanji.korean_r.as_ref().unwrap();

            Some(
                korean_h
                    .iter()
                    .zip(korean_r.iter())
                    .map(|(h, k)| format!("{} ({})", h, k))
                    .collect(),
            )
        } else {
            None
        }
    }

    // TODO translate this one
    #[inline]
    pub fn get_parts_title(&self) -> &'static str {
        if self
            .kanji
            .parts
            .as_ref()
            .map(|i| i.len())
            .unwrap_or_default()
            > 1
        {
            "Parts"
        } else {
            "Part"
        }
    }

    #[inline]
    pub fn get_radical(&self) -> String {
        if let Some(ref alternative) = self.kanji.radical.alternative {
            format!("{} ({})", self.kanji.radical.literal, alternative)
        } else {
            self.kanji.radical.literal.clone().to_string()
        }
    }

    #[inline]
    pub fn get_rad_len(&self) -> usize {
        self.kanji
            .radical
            .alternative
            .as_ref()
            .map(|_| 1)
            .unwrap_or_default()
            + self
                .kanji
                .radical
                .translations
                .as_ref()
                .map(|i| i.join(", ").len())
                .unwrap_or_default()
    }
}
