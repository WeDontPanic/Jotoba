use std::{fs::read_to_string, vec};

use types::jotoba::{
    kanji::Kanji,
    languages::Language,
    words::{filter_languages, Word},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub kanji: Kanji,
    pub kun_dicts: Option<Vec<Word>>,
    pub on_dicts: Option<Vec<Word>>,
}

impl Item {
    pub fn load_words(k: Kanji, lang: Language, show_english: bool) -> Self {
        let kun_dicts = load_dicts(&k.kun_dicts, lang, show_english);
        let on_dicts = load_dicts(&k.on_dicts, lang, show_english);

        Self {
            kun_dicts,
            on_dicts,
            kanji: k,
        }
    }
}

fn load_dicts(dicts: &Option<Vec<u32>>, lang: Language, show_english: bool) -> Option<Vec<Word>> {
    let word_storage = resources::get().words();

    let loaded_dicts = dicts.as_ref().map(|i| {
        let mut words: Vec<_> = i
            .iter()
            .filter_map(|j| word_storage.by_sequence(*j))
            .cloned()
            .collect();

        filter_languages(words.iter_mut(), lang, show_english);

        words
    });

    loaded_dicts
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

    /// Returns the amount of parts a kanji is bulit with
    #[inline]
    pub fn get_parts_count(&self) -> usize {
        self.kanji
            .parts
            .as_ref()
            .map(|i| i.len())
            .unwrap_or_default()
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
