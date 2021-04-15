#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Word(word::Item),
}

/// Defines a word result item
pub mod word {
    use std::cmp::Ordering;

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
    #[derive(Debug, Clone, Default)]
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

    // Small handy functions used in the templates

    impl Item {
        pub fn alt_readings_beautified(&self) -> String {
            self.reading
                .alternative
                .iter()
                .map(|i| i.reading.clone())
                .join(", ")
        }

        pub fn is_common(&self) -> bool {
            self.reading.get_reading().priorities.is_some()
        }

        pub fn priorities_count(&self) -> usize {
            self.priorities
                .as_ref()
                .map(|i| i.len())
                .unwrap_or_default()
        }

        pub fn get_reading(&self) -> &Dict {
            return self.reading.get_reading();
        }

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
    }

    impl Reading {
        pub fn get_reading(&self) -> &Dict {
            self.kanji.as_ref().unwrap_or(self.kana.as_ref().unwrap())
        }

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

    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.sequence == other.sequence
        }
    }

    impl std::cmp::Eq for Item {}

    impl std::cmp::PartialOrd for Item {
        fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
            // Always put common words at the top
            if self.is_common() && !other.is_common() {
                Some(Ordering::Less)
            } else if self.reading.kana.is_some() && other.reading.kana.is_some() {
                // If both have kana reading
                let self_read = self.reading.get_reading();
                let other_read = other.reading.get_reading();

                if self.priorities_count() > other.priorities_count() {
                    Some(Ordering::Less)
                } else if self_read.reading.len() < other_read.reading.len() {
                    // Order by length
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            } else {
                // If one doesn't have kana reading
                None
            }
        }
    }

    impl std::cmp::Ord for Item {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap_or_else(|| {
                println!("unwrapped");
                Ordering::Greater
            })
        }
    }
}
