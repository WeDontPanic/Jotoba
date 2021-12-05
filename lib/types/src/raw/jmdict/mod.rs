use crate::jotoba::{
    languages::Language,
    words::{
        dialect::Dialect, field::Field, gtype::GType, information::Information, misc::Misc,
        part_of_speech::PartOfSpeech, priority::Priority, sense::Gairaigo,
    },
};

use serde::{Deserialize, Serialize};

/// An dict entry. Represents one word, phrase or expression
#[derive(Debug, Default, Clone)]
pub struct Entry {
    pub sequence: u32,
    /// Different readings of a word
    pub elements: Vec<EntryElement>,
    /// Translations into various languages
    pub senses: Vec<EntrySense>,
}

/// A single element for an entry. Defines reading, kanji and additional
/// information for the japanese word
#[derive(Debug, Default, Clone)]
pub struct EntryElement {
    /// Is kanji reading
    pub kanji: bool,
    /// The reading
    pub value: String,
    pub priorities: Vec<Priority>,
    pub reading_info: Vec<Information>,
    pub no_true_reading: bool,
}

/// A single 'sense' item for an entry
#[derive(Debug, Default, Clone)]
pub struct EntrySense {
    pub id: u8,
    pub glosses: Vec<GlossValue>,
    pub misc: Option<Misc>,
    pub part_of_speech: Vec<PartOfSpeech>,
    pub antonym: Option<String>,
    pub field: Option<Field>,
    pub xref: Option<String>,
    pub dialect: Option<Dialect>,
    pub information: Option<String>,
    pub gairaigo: Option<Gairaigo>,
    pub example_sentence: Option<u32>,
}

impl EntrySense {
    pub fn clear(&mut self) {
        self.glosses.clear();

        if let Some(ref mut ant) = self.antonym {
            ant.clear();
            self.antonym = None;
        }

        if let Some(ref mut information) = self.information {
            information.clear();
            self.information = None;
        }

        if let Some(ref mut xref) = self.xref {
            xref.clear();
            self.xref = None;
        }

        self.field = None;
        self.dialect = None;
        self.misc = None;
        self.part_of_speech.clear();
        self.example_sentence = None;
        self.gairaigo = None;
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Hash)]
pub struct Translation {
    pub language: Language,
    pub value: String,
}

/// A single gloss entry.
#[derive(Debug, Clone, PartialEq)]
pub struct GlossValue {
    pub language: Language,
    pub g_type: Option<GType>,
    pub value: String,
}
