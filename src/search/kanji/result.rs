use std::{fs::read_to_string, path::Path, vec};

use futures::try_join;

use crate::{
    error::Error,
    models::{kanji::KanjiResult, radical::Radical},
    parse::jmdict::languages::Language,
    search::word::{result::Word, WordSearch},
    utils::{self, to_option},
    DbPool,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    pub kanji: KanjiResult,
    pub kun_dicts: Option<Vec<Word>>,
    pub on_dicts: Option<Vec<Word>>,
    pub radical: Radical,
    pub parts: Option<Vec<String>>,
}

impl Item {
    /// Convert a DbKanji to Item
    ///
    /// Required because the kanji's reading-componds
    /// aren't loaded by default due it being an array
    pub async fn from_db(
        db: &DbPool,
        k: KanjiResult,
        lang: Language,
        show_english: bool,
    ) -> Result<Self, Error> {
        let kun_dicts = k.kanji.kun_dicts.clone().unwrap_or_default();

        let (radical, parts, kun_words): (Radical, Vec<String>, Vec<Word>) = try_join!(
            k.kanji.load_radical(db),
            k.kanji.load_parts(db),
            WordSearch::load_words_by_seq(db, &kun_dicts, lang, show_english, &None)
        )?;

        let loaded_kd = kun_words
            .into_iter()
            // Filter english items if user don't want to se them
            .filter(|i| show_english || !i.senses.is_empty())
            .collect();

        Ok(Self {
            kanji: k,
            kun_dicts: utils::to_option(loaded_kd),
            on_dicts: None,
            radical,
            parts: to_option(parts),
        })
    }
}

impl Item {
    /// Print kanji grade pretty for frontend
    pub fn school_str(&self) -> Option<String> {
        self.kanji.kanji.school_str()
    }

    pub fn get_animation_path(&self) -> String {
        format!("html/assets/svg/{}_animated.svgs", self.kanji.kanji.literal)
    }

    pub fn get_stroke_frames_url(&self) -> String {
        format!("assets/svg/{}_frames.svg", self.kanji.kanji.literal)
    }

    // Returns true if the kanji has a stroke animation file
    pub fn has_animation_file(&self) -> bool {
        Path::new(&self.get_animation_path()).exists()
    }

    // Returns true if the kanji has stroke frames
    pub fn has_stroke_frames(&self) -> bool {
        Path::new(&self.get_animation_path()).exists()
    }

    /// Return the animation entries for the template
    pub fn get_animation_entries(&self) -> Vec<(String, String)> {
        if let Ok(content) = read_to_string(self.get_animation_path()) {
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
        if self.kanji.kanji.korean_r.is_some() && self.kanji.kanji.korean_h.is_some() {
            let korean_h = self.kanji.kanji.korean_h.as_ref().unwrap();
            let korean_r = self.kanji.kanji.korean_r.as_ref().unwrap();

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

    pub fn get_parts_title(&self) -> &'static str {
        if self.parts.as_ref().map(|i| i.len()).unwrap_or_default() > 1 {
            "Parts"
        } else {
            "Part"
        }
    }

    pub fn get_radical(&self) -> String {
        if let Some(ref alternative) = self.radical.alternative {
            format!("{} ({})", self.radical.literal, alternative)
        } else {
            self.radical.literal.clone()
        }
    }

    pub fn get_rad_len(&self) -> usize {
        self.radical.literal.len()
            + self
                .radical
                .alternative
                .as_ref()
                .map(|i| i.len())
                .unwrap_or_default()
            + self
                .radical
                .translations
                .as_ref()
                .map(|i| i.join(", ").len())
                .unwrap_or_default()
    }
}
