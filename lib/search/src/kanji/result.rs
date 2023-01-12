use std::{fs::read_to_string, path::Path};
use types::jotoba::{
    kanji::Kanji,
    language::param::AsLangParam,
    words::{filter_languages, Word},
};

// The final result of a Kanji search
#[derive(Default)]
pub struct KanjiResult {
    pub items: Vec<Item>,
    pub total_len: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub kanji: Kanji,
    pub kun_dicts: Option<Vec<Word>>,
    pub on_dicts: Option<Vec<Word>>,
    pub has_compositions: bool,
}

impl Item {
    pub fn load_words(k: Kanji, lang: impl AsLangParam) -> Self {
        let kun_dicts = load_dicts(&k.kun_dicts, lang);
        let on_dicts = load_dicts(&k.on_dicts, lang);

        let has_compositions = resources::get().kanji().ids(k.literal).is_some();

        Self {
            kun_dicts,
            on_dicts,
            kanji: k,
            has_compositions,
        }
    }
}

fn load_dicts(dicts: &Vec<u32>, lang: impl AsLangParam) -> Option<Vec<Word>> {
    let word_storage = resources::get().words();
    let mut words: Vec<_> = dicts
        .iter()
        .filter_map(|j| word_storage.by_sequence(*j))
        .cloned()
        .collect();

    filter_languages(words.iter_mut(), lang);

    if words.is_empty() {
        return None;
    }

    Some(words)
}

impl Item {
    /// Returns the entries' frames (svg)
    pub fn get_frames<P: AsRef<Path>>(&self, assets_path: P) -> Option<String> {
        read_to_string(self.kanji.get_stroke_frames_path(assets_path)).ok()
    }

    /// Return the animation entries for the template
    pub fn get_animation<P: AsRef<Path>>(&self, assets_path: P) -> Option<String> {
        read_to_string(self.kanji.get_animation_path(assets_path)).ok()
    }

    /// Get a list of korean readings, formatted as: "<Hangul> (<romanized>)"
    pub fn get_korean(&self) -> Option<Vec<String>> {
        if !self.kanji.korean_r.is_empty() && !self.kanji.korean_h.is_empty() {
            let korean_h = &self.kanji.korean_h;
            let korean_r = &self.kanji.korean_r;

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
        self.kanji.parts.len()
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
