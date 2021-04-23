use super::query_parser::QueryType;

/// A single user provided query in a
/// parsed format
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Query {
    pub items: Vec<String>,
    pub type_: QueryType,
    pub tags: Vec<Tag>,
    pub form: Form,
    pub language: QueryLang,
}

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tag {
    // Word search
    Noun,
    Adverb,
    Sfx,
    Verb,
    Adjective,

    // Search types
    Kanji,
    Sentence,
    Name,
    Word,
}

/// The language of the query
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QueryLang {
    Japanese,
    Foreign,
    Undetected,
}

/// The form the query was provided in
#[derive(Debug, Clone, PartialEq)]
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

/// A kanji-reading search
#[derive(Debug, Clone, PartialEq)]
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
        Some(match s[1..].to_lowercase().as_str() {
            "noun" => Self::Noun,
            "adverb" => Self::Adverb,
            "sfx" => Self::Sfx,
            "verb" => Self::Verb,
            "adjective" => Self::Adjective,

            "kanji" => Self::Kanji,
            "sentence" => Self::Sentence,
            "name" => Self::Name,
            "word" => Self::Word,
            _ => return None,
        })
    }
}

impl Query {
    pub fn is_valid(&self) -> bool {
        !self.items.is_empty()
    }
}
