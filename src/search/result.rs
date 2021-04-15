#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Word(word::Item),
}

/// Defines a word result item
pub mod word {
    use itertools::Itertools;

    use crate::{
        models::{dict::Dict, sense::Sense as DbSenseEntry},
        parse::jmdict::{
            dialect::Dialect, field::Field, gtype::GType, information::Information,
            languages::Language, misc::Misc, part_of_speech::PartOfSpeech, priority::Priority,
        },
    };

    /// A single word item
    #[derive(Debug, Clone, PartialEq, Default)]
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

    impl Item {
        pub fn alt_readings_beautified(&self) -> String {
            self.reading
                .alternative
                .iter()
                .map(|i| i.reading.clone())
                .join(", ")
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
