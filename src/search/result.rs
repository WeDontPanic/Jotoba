use crate::models::kanji::Kanji;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Word(word::Item),
    Kanji(Kanji),
}

impl From<word::Item> for Item {
    fn from(k: word::Item) -> Self {
        Self::Word(k)
    }
}

impl From<Kanji> for Item {
    fn from(k: Kanji) -> Self {
        Self::Kanji(k)
    }
}

pub mod kanji {
    use search::word::WordSearch;

    use super::word::Item as WordItem;
    use std::{fs::read_to_string, path::Path, vec};

    use crate::{
        models::kanji::Kanji as DbKanji, parse::jmdict::languages::Language, search, utils, DbPool,
    };

    #[derive(Debug, PartialEq, Clone)]
    pub struct Item {
        pub kanji: DbKanji,
        pub kun_dicts: Option<Vec<WordItem>>,
        pub on_dicts: Option<Vec<WordItem>>,
    }

    impl Item {
        pub async fn from_db(db: &DbPool, k: DbKanji) -> Self {
            let kun_dicts = k.kun_dicts.clone().unwrap_or_default();

            let loaded_kd = WordSearch::load_words_by_seq(db, &kun_dicts, Language::German)
                .await
                .unwrap();

            Self {
                kanji: k,
                kun_dicts: utils::to_option(loaded_kd),
                on_dicts: None,
            }
        }
    }

    impl Item {
        /// Print kanji grade pretty for frontend
        pub fn school_str(&self) -> Option<String> {
            self.kanji.school_str()
        }

        pub fn get_animation_path(&self) -> String {
            format!("html/assets/svg/{}_animated.svgs", self.kanji.literal)
        }

        pub fn get_stroke_frames_url(&self) -> String {
            format!("assets/svg/{}_frames.svg", self.kanji.literal)
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
                    .split("\n")
                    .into_iter()
                    .map(|i| {
                        let mut s = i.split(";");
                        (s.next().unwrap().to_owned(), s.next().unwrap().to_owned())
                    })
                    .collect::<Vec<(String, String)>>()
            } else {
                vec![]
            }
        }
    }
}

/// Defines a word result item
pub mod word {
    use crate::{japanese::JapaneseExt, parse::jmdict::languages::Language, utils::to_option};
    use itertools::Itertools;

    use crate::{
        japanese::{self, SentencePart},
        models::{dict::Dict, sense::Sense as DbSenseEntry},
        parse::jmdict::{
            dialect::Dialect, field::Field, gtype::GType, information::Information, misc::Misc,
            part_of_speech::PartOfSpeech, priority::Priority,
        },
        utils,
    };

    /// A single word item
    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct Item {
        pub sequence: i32,
        pub priorities: Option<Vec<Priority>>,
        pub information: Option<Vec<Information>>,
        pub reading: Reading,
        pub senses: Vec<Sense>,
    }

    /// Various readins of a word
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Reading {
        pub sequence: i32,
        pub kana: Option<Dict>,
        pub kanji: Option<Dict>,
        pub alternative: Vec<Dict>,
    }

    /// A single sense for a word. Represents one language,
    /// one misc item and 1..n glosses
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Sense {
        pub language: Language,
        pub misc: Option<Misc>,
        pub field: Option<Field>,
        pub dialect: Option<Dialect>,
        pub glosses: Vec<Gloss>,
        pub xref: Option<String>,
        pub antonym: Option<String>,
        pub information: Option<String>,
    }

    /// A gloss value represents one word in the
    /// translated language.
    #[derive(Debug, Clone, PartialEq, Default)]
    pub struct Gloss {
        pub gloss: String,
        pub g_type: Option<GType>,
        pub part_of_speech: Vec<PartOfSpeech>,
    }

    impl From<Vec<DbSenseEntry>> for Sense {
        fn from(entry: Vec<DbSenseEntry>) -> Self {
            let first = &entry[0];
            let gtype = &first.gtype;
            Sense {
                language: first.language,
                misc: first.misc,
                field: first.field,
                dialect: first.dialect,
                xref: first.xref.clone(),
                antonym: first.antonym.clone(),
                information: first.information.clone(),
                glosses: entry
                    .clone()
                    .into_iter()
                    .map(|i| Gloss {
                        part_of_speech: i.part_of_speech.unwrap_or_default(),
                        g_type: (*gtype),
                        gloss: i.gloss,
                    })
                    .collect_vec(),
            }
        }
    }

    //
    // Small handy functions used in the templates //
    //

    impl Item {
        /// Get alternative readings in a beautified, print-ready format
        pub fn alt_readings_beautified(&self) -> String {
            self.reading
                .alternative
                .iter()
                .map(|i| i.reading.clone())
                .join(", ")
        }

        /// Returns true if a word is common
        pub fn is_common(&self) -> bool {
            self.reading.get_reading().priorities.is_some()
        }

        /// Return the amount of priorities a word has
        pub fn priorities_count(&self) -> usize {
            self.priorities
                .as_ref()
                .map(|i| i.len())
                .unwrap_or_default()
        }

        /// Returns the reading of a word
        pub fn get_reading(&self) -> &Dict {
            return self.reading.get_reading();
        }

        pub fn glosses_pretty(&self) -> String {
            let senses = self.get_senses();

            if !senses[0].is_empty() {
                Self::pretty_print_senses(&senses[0])
            } else {
                Self::pretty_print_senses(&senses[1])
            }
        }

        fn pretty_print_senses(senses: &Vec<Sense>) -> String {
            senses
                .iter()
                .map(|i| i.glosses.clone())
                .flatten()
                .into_iter()
                .map(|i| i.gloss)
                .join(", ")
        }

        /// Returns furigana reading-pairs of an Item
        pub fn get_furigana(&self) -> Option<Vec<SentencePart>> {
            if self.reading.kanji.is_some() && self.reading.kana.is_some() {
                japanese::furigana_pairs(
                    self.reading
                        .kanji
                        .as_ref()
                        .map(|i| i.reading.as_str())
                        .unwrap(),
                    self.reading
                        .kana
                        .as_ref()
                        .map(|i| i.reading.as_str())
                        .unwrap(),
                )
            } else {
                None
            }
        }

        /// Return true if item has a certain reading
        pub fn has_reading(&self, reading: &str, ignore_case: bool) -> bool {
            if let Some(kanji) = self.reading.kanji.as_ref().map(|i| &i.reading) {
                if (ignore_case && kanji.to_lowercase() == reading.to_lowercase())
                    || (kanji == reading)
                {
                    return true;
                }
            }

            if let Some(kana) = self.reading.kana.as_ref().map(|i| &i.reading) {
                if (ignore_case && kana.to_lowercase() == reading.to_lowercase())
                    || (kana == reading)
                {
                    return true;
                }
            }
            false
        }

        /// Get senses ordered by language (non-english first)
        pub fn get_senses(&self) -> Vec<Vec<Sense>> {
            let (english, mut other): (Vec<Sense>, Vec<Sense>) = self
                .senses
                .clone()
                .into_iter()
                .partition(|i| i.language == Language::English);

            // Set other's p_o_s items to the ones from english which are available in each sense
            let unionized_pos = Self::pos_unionized(&english);
            if !unionized_pos.is_empty() {
                other.iter_mut().for_each(|item| {
                    for gloss in item.glosses.iter_mut() {
                        gloss.part_of_speech = unionized_pos.clone();
                    }
                });
            }

            // Set other's misc items to the ones from english if they are all the same
            if Self::all_misc_same(&english) && !english.is_empty() && english[0].misc.is_some() {
                let misc = english[0].misc;
                other.iter_mut().for_each(|item| {
                    item.misc = misc;
                });
            }

            vec![other, english]
        }

        /// Return all senses of a language
        pub fn senses_by_lang(&self, language: Language) -> Option<Vec<Sense>> {
            to_option(
                self.senses
                    .iter()
                    .filter(|i| i.language == language)
                    .cloned()
                    .collect_vec(),
            )
        }

        /// Return true if all 'misc' items are of the same value
        pub fn all_misc_same(senses: &Vec<Sense>) -> bool {
            if senses.is_empty() || senses[0].misc.is_none() {
                return false;
            }

            let mut sense_iter = senses.iter();
            let first_misc = sense_iter.next().unwrap().misc;

            for i in sense_iter {
                if i.misc != first_misc {
                    return false;
                }
            }

            true
        }

        /// Return true if all 'part_of_speech' items are of the same value
        pub fn pos_unionized(senses: &Vec<Sense>) -> Vec<PartOfSpeech> {
            if senses.is_empty()
                || senses[0].glosses.is_empty()
                || senses[0].glosses[0].part_of_speech.is_empty()
            {
                return vec![];
            }

            let mut sense_iter = senses.iter();
            let mut pos = sense_iter.next().unwrap().glosses[0].part_of_speech.clone();

            for i in sense_iter {
                pos = utils::union_elements(&pos, &i.glosses[0].part_of_speech)
                    .into_iter()
                    .cloned()
                    .collect_vec();
            }

            pos
        }

        /// Get amount of tags which will be displayed below the reading
        pub fn get_word_tag_count(&self) -> u8 {
            let mut c = 0;
            if self.is_common() {
                c += 1;
            }

            if self.get_reading().jlpt_lvl.is_some() {
                c += 1;
            }

            c
        }

        /// Return true if item is a katakana word
        pub fn is_katakana_word(&self) -> bool {
            self.reading.is_katakana()
        }
    }

    impl Reading {
        /// Return true if reading represents a katakana only word
        pub fn is_katakana(&self) -> bool {
            self.kana.as_ref().unwrap().reading.is_katakana() && self.kanji.is_none()
        }

        /// Returns the word-reading of a Reading object
        pub fn get_reading(&self) -> &Dict {
            self.kanji.as_ref().unwrap_or(self.kana.as_ref().unwrap())
        }

        /// Returns the jplt level of a word. None if
        /// a word doesn't have a JPLT lvl assigned
        pub fn get_jplt_lvl(&self) -> Option<i32> {
            self.get_reading().jlpt_lvl
        }
    }

    impl Sense {
        // Get a senses tags prettified
        pub fn get_glosses(&self) -> String {
            self.glosses.iter().map(|i| i.gloss.clone()).join("; ")
        }

        // Get a senses misc entries
        pub fn get_misc(&self) -> Option<String> {
            self.misc.map(|i| {
                let s: String = i.into();
                s
            })
        }

        // Get a senses tags prettified
        pub fn get_parts_of_speech(&self) -> String {
            self.glosses[0]
                .part_of_speech
                .iter()
                .map(|i| i.humanized())
                .join(", ")
        }
    }
}

#[cfg(test)]
mod test {
    use super::word::*;
    use crate::japanese::JapaneseExt;
    use crate::models::dict::Dict;

    #[test]
    fn test_katakana() {
        let reading = Reading {
            kana: Some(Dict {
                reading: String::from("カタカナ"),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert!(reading.kana.as_ref().unwrap().reading.is_katakana());
        assert!(reading.is_katakana());
    }
}
