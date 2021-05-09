use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};

use itertools::Itertools;

use crate::parse::jmdict::{languages::Language, part_of_speech::PosSimple};

use super::query_parser::QueryType;

/// In-cookie saved personalized settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserSettings {
    pub user_lang: Language,
    pub show_english: bool,
    pub english_on_top: bool,
}

impl Hash for UserSettings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user_lang.hash(state);
        self.show_english.hash(state);
    }
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            show_english: true,
            user_lang: Language::default(),
            english_on_top: false,
        }
    }
}

/// A single user provided query in a
/// parsed format
#[derive(Debug, Clone, PartialEq, Default, Hash)]
pub struct Query {
    pub original_query: String,
    pub query: String,
    pub type_: QueryType,
    pub tags: Vec<Tag>,
    pub form: Form,
    pub language: QueryLang,
    pub settings: UserSettings,
    pub page: usize,
    pub word_index: usize,
    pub parse_japanese: bool,
}

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum SearchTypeTag {
    Kanji,
    Sentence,
    Name,
    Word,
}

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Tag {
    SearchType(SearchTypeTag),
    PartOfSpeech(PosSimple),
}

/// The language of the query
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum QueryLang {
    Japanese,
    Foreign,
    Undetected,
}

/// The form the query was provided in
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Form {
    /// A single word was provided
    SingleWord,
    /// Multiple words were provided
    MultiWords,
    /// Kanji reading based search eg. '気 ケ'
    KanjiReading(KanjiReading),
    /// Form was not recognized
    Undetected,
}

impl Form {
    pub fn as_kanji_reading(&self) -> Option<&KanjiReading> {
        if let Self::KanjiReading(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

/// A kanji-reading search
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct KanjiReading {
    /// The provided kanji literal
    pub literal: char,
    /// The provided kanji reading
    pub reading: String,
}

impl KanjiReading {
    pub fn new(literal: &str, reading: &str) -> KanjiReading {
        KanjiReading {
            literal: literal.chars().next().unwrap(),
            reading: reading.to_string(),
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::Undetected
    }
}

impl Default for QueryLang {
    fn default() -> Self {
        Self::Undetected
    }
}

impl Tag {
    // Parse a tag from a string
    pub fn from_str(s: &str) -> Option<Tag> {
        Some(if let Some(tag) = Self::parse_search_type(s) {
            tag
        } else {
            match PosSimple::from_str(&s[1..]) {
                Ok(pos) => Self::PartOfSpeech(pos),
                Err(_) => return None,
            }
        })
    }

    /// Parse only search type
    pub fn parse_search_type(s: &str) -> Option<Tag> {
        Some(match s[1..].to_lowercase().as_str() {
            "kanji" => Self::SearchType(SearchTypeTag::Kanji),
            "sentence" | "sentences" => Self::SearchType(SearchTypeTag::Sentence),
            "name" | "names" => Self::SearchType(SearchTypeTag::Name),
            "word" | "words" => Self::SearchType(SearchTypeTag::Word),
            _ => return None,
        })
    }

    /// Returns `true` if the tag is [`SearchType`].
    pub fn is_search_type(&self) -> bool {
        matches!(self, Self::SearchType(..))
    }

    /// Returns `true` if the tag is [`PartOfSpeech`].
    pub fn is_part_of_speech(&self) -> bool {
        matches!(self, Self::PartOfSpeech(..))
    }

    pub fn as_search_type(&self) -> Option<&SearchTypeTag> {
        if let Self::SearchType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_part_of_speech(&self) -> Option<&PosSimple> {
        if let Self::PartOfSpeech(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl Query {
    pub fn is_valid(&self) -> bool {
        !self.query.is_empty()
    }

    pub fn get_hash(&self) -> u64 {
        let mut hash = DefaultHasher::new();
        self.hash(&mut hash);
        hash.finish()
    }

    /// Returns true if the query has at least one pos tag
    pub fn has_part_of_speech_tags(&self) -> bool {
        !self.get_part_of_speech_tags().is_empty()
    }

    /// Returns all search type tags
    pub fn get_search_type_tags(&self) -> Vec<SearchTypeTag> {
        self.tags
            .iter()
            .filter(|i| i.is_search_type())
            .map(|i| i.as_search_type().unwrap())
            .copied()
            .collect()
    }

    /// Returns all PosSimple tags
    pub fn get_part_of_speech_tags(&self) -> Vec<PosSimple> {
        self.tags
            .iter()
            .filter(|i| i.is_part_of_speech())
            .map(|i| i.as_part_of_speech().unwrap())
            .copied()
            .collect()
    }

    /// Returns the original_query with search type tags omitted
    pub fn without_search_type_tags(&self) -> String {
        self.original_query
            .clone()
            .split(" ")
            .into_iter()
            .filter(|i| {
                // Filter out all search type tags
                (i.starts_with("#") && Tag::parse_search_type(i).is_none()) || !i.starts_with("#")
            })
            .join(" ")
            .to_owned()
    }
}
