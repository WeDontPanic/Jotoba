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

/// Defines a word result item
pub mod word {
    use itertools::Itertools;

    use crate::{
        japanese::{self, SentencePart},
        models::{dict::Dict, sense::Sense as DbSenseEntry},
        parse::jmdict::{
            dialect::Dialect, field::Field, gtype::GType, information::Information,
            languages::Language, misc::Misc, part_of_speech::PartOfSpeech, priority::Priority,
        },
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
    }

    impl Reading {
        /// Returns the word-reading of a Reading object
        pub fn get_reading(&self) -> &Dict {
            self.kanji.as_ref().unwrap_or(self.kana.as_ref().unwrap())
        }

        /// Returns the jplt level of a word. None if
        /// a word doesn't have a JPLT lvl assigned
        pub fn get_jplt_lvl(&self) -> Option<u8> {
            // TODO
            None
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
        pub fn get_tags(&self) -> String {
            self.glosses[0]
                .part_of_speech
                .iter()
                .map(|i| {
                    let s: String = i.clone().into();
                    s
                })
                .join(", ")
        }
    }
}
