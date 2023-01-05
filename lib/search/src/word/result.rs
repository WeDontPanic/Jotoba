use types::jotoba::words::inflection::Inflection;

use crate::executor::out_builder::OutputAddable;

#[derive(Default, Clone, Debug)]
pub struct AddResData {
    pub sentence: Option<SentenceInfo>,
    pub inflection: Option<InflectionInformation>,
    pub raw_query: String,
    pub number: Option<String>,
}

impl OutputAddable for AddResData {
    #[inline]
    fn is_empty(&self) -> bool {
        self.sentence.is_none() && self.inflection.is_none()
    }
}

#[derive(Default, Clone, Debug)]
pub struct SentenceInfo {
    pub parts: Option<sentence_reader::Sentence>,
    pub index: usize,
    pub query: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InflectionInformation {
    /// Normalized form of the word
    pub lexeme: String,
    /// All inflections
    pub inflections: Vec<Inflection>,
}

impl AddResData {
    pub fn has_sentence(&self) -> bool {
        self.sentence.is_some()
    }

    pub fn has_inflection(&self) -> bool {
        self.inflection.is_some()
    }

    pub fn sentence_parts(&self) -> Option<&sentence_reader::Sentence> {
        self.sentence.as_ref().and_then(|i| i.parts.as_ref())
    }

    pub fn sentence_index(&self) -> usize {
        self.sentence.as_ref().map(|i| i.index).unwrap_or(0)
    }
}

impl InflectionInformation {
    pub fn from_part(part: &sentence_reader::Part) -> Option<Self> {
        if !part.has_inflections() {
            return None;
        }

        Some(InflectionInformation {
            lexeme: part.get_normalized(),
            inflections: part.inflections().to_vec(),
        })
    }
}

pub fn selected(curr: usize, selected: usize) -> &'static str {
    if curr == selected {
        "selected"
    } else {
        ""
    }
}
