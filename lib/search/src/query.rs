use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::{query_parser, regex_query::RegexSQuery};

use itertools::Itertools;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use types::jotoba::{
    kanji,
    languages::Language,
    search::QueryType,
    words::{misc::Misc, part_of_speech::PosSimple},
};

const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS.add(b'/');

/// A single user provided query in a parsed format
#[derive(Debug, Clone, PartialEq, Default, Hash)]
pub struct Query {
    pub original_query: String,
    pub query: String,
    pub type_: QueryType,
    pub tags: Vec<Tag>,
    pub form: Form,
    pub language: QueryLang,
    pub settings: UserSettings,
    pub page_offset: usize,
    pub page: usize,
    pub word_index: usize,
    pub parse_japanese: bool,
    pub language_override: Option<Language>,
    /// Whether to use the user query only or modify it if necessary
    pub use_original: bool,
}

/// In-cookie saved personalized settings
#[derive(Debug, Clone, Copy)]
pub struct UserSettings {
    pub user_lang: Language,
    pub page_lang: localization::language::Language,
    pub show_english: bool,
    pub english_on_top: bool,
    pub cookies_enabled: bool,
    pub page_size: u32,
    pub kanji_page_size: u32,
    pub show_example_sentences: bool,
    pub sentence_furigana: bool,
}

impl PartialEq for UserSettings {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.user_lang == other.user_lang && self.show_english == other.show_english
    }
}

impl Hash for UserSettings {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.user_lang.hash(state);
        self.show_english.hash(state);
    }
}

impl Default for UserSettings {
    #[inline]
    fn default() -> Self {
        Self {
            show_english: true,
            user_lang: Language::default(),
            page_lang: localization::language::Language::default(),
            english_on_top: false,
            cookies_enabled: false,
            page_size: 10,
            kanji_page_size: 4,
            show_example_sentences: true,
            sentence_furigana: true,
        }
    }
}

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Tag {
    SearchType(SearchTypeTag),
    PartOfSpeech(PosSimple),
    Misc(Misc),
    Jlpt(u8),
    GenkiLesson(u8),
    Hidden,
    IrregularIruEru,
}

/// Hashtag based search tags
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum SearchTypeTag {
    Kanji,
    Sentence,
    Name,
    Word,
}

/// The language of the query
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum QueryLang {
    Japanese,
    Foreign,
    Korean,
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
    KanjiReading(kanji::ReadingSearch),
    /// Tag only. Implies query string to be empty
    TagOnly,
    /// Form was not recognized
    Undetected,
}

impl Form {
    #[inline]
    pub fn as_kanji_reading(&self) -> Option<&kanji::ReadingSearch> {
        if let Self::KanjiReading(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the form is [`KanjiReading`].
    #[inline]
    pub fn is_kanji_reading(&self) -> bool {
        matches!(self, Self::KanjiReading(..))
    }

    /// Returns `true` if the form is [`TagOnly`].
    ///
    /// [`TagOnly`]: Form::TagOnly
    #[inline]
    pub fn is_tag_only(&self) -> bool {
        matches!(self, Self::TagOnly)
    }
}

impl Default for Form {
    #[inline]
    fn default() -> Self {
        Self::Undetected
    }
}

impl Default for QueryLang {
    #[inline]
    fn default() -> Self {
        Self::Undetected
    }
}

impl Tag {
    /// Parse a tag from a string
    pub fn parse_from_str(s: &str) -> Option<Tag> {
        if let Some(tag) = s.to_lowercase().strip_prefix("#") {
            match tag {
                "hidden" | "hide" => return Some(Tag::Hidden),
                "irrichidan" | "irregularichidan" | "irregular-ichidan" => {
                    return Some(Tag::IrregularIruEru)
                }
                _ => (),
            }
        }

        #[allow(irrefutable_let_patterns)]
        if let Some(tag) = Self::parse_genki_tag(s) {
            return Some(tag);
        } else if let Some(tag) = Self::parse_jlpt_tag(s) {
            return Some(tag);
        } else if let Some(tag) = Self::parse_search_type(s) {
            return Some(tag);
        } else {
            match PosSimple::from_str(&s[1..]) {
                Ok(pos) => return Some(Self::PartOfSpeech(pos)),
                _ => return None,
            }
        }
    }

    /// Returns `Some(u8)` if `s` is a valid N-tag
    fn parse_jlpt_tag(s: &str) -> Option<Tag> {
        if s.chars().skip(1).next()?.to_lowercase().next()? != 'n' {
            return None;
        }

        let nr: u8 = s[2..].parse().ok()?;
        (nr > 0 && nr < 6).then(|| Tag::Jlpt(nr))
    }

    /// Returns `Some(u8)` if `s` is a valid genki-tag
    fn parse_genki_tag(s: &str) -> Option<Tag> {
        let e = s.trim().strip_prefix("#")?.trim().to_lowercase();
        if !e.starts_with("genki") {
            return None;
        }

        let nr: u8 = s[6..].parse().ok()?;
        (nr >= 3 && nr <= 23).then(|| Tag::GenkiLesson(nr))
    }

    /// Parse only search type
    fn parse_search_type(s: &str) -> Option<Tag> {
        Some(match s[1..].to_lowercase().as_str() {
            "kanji" => Self::SearchType(SearchTypeTag::Kanji),
            "sentence" | "sentences" => Self::SearchType(SearchTypeTag::Sentence),
            "name" | "names" => Self::SearchType(SearchTypeTag::Name),
            "word" | "words" => Self::SearchType(SearchTypeTag::Word),
            "abbreviation" | "abbrev" => Self::Misc(Misc::Abbreviation),
            _ => return None,
        })
    }

    /// Returns true if the tag is allowed to be used without a query
    #[inline]
    pub fn is_empty_allowed(&self) -> bool {
        self.is_jlpt() || self.is_genki_lesson() || self.is_irregular_iru_eru()
    }

    /// Returns `true` if the tag is [`SearchType`].
    #[inline]
    pub fn is_search_type(&self) -> bool {
        matches!(self, Self::SearchType(..))
    }

    /// Returns `true` if the tag is [`PartOfSpeech`].
    #[inline]
    pub fn is_part_of_speech(&self) -> bool {
        matches!(self, Self::PartOfSpeech(..))
    }

    #[inline]
    pub fn as_search_type(&self) -> Option<&SearchTypeTag> {
        if let Self::SearchType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_part_of_speech(&self) -> Option<&PosSimple> {
        if let Self::PartOfSpeech(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`Misc`].
    ///
    /// [`Misc`]: Tag::Misc
    #[inline]
    pub fn is_misc(&self) -> bool {
        matches!(self, Self::Misc(..))
    }

    #[inline]
    pub fn as_misc(&self) -> Option<&Misc> {
        if let Self::Misc(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`Jlpt`].
    ///
    /// [`Jlpt`]: Tag::Jlpt
    #[inline]
    pub fn is_jlpt(&self) -> bool {
        matches!(self, Self::Jlpt(..))
    }

    #[inline]
    pub fn as_jlpt(&self) -> Option<&u8> {
        if let Self::Jlpt(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`GenkiLesson`].
    ///
    /// [`GenkiLesson`]: Tag::GenkiLesson
    pub fn is_genki_lesson(&self) -> bool {
        matches!(self, Self::GenkiLesson(..))
    }

    pub fn as_genki_lesson(&self) -> Option<&u8> {
        if let Self::GenkiLesson(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the tag is [`IrregularIruEru`].
    ///
    /// [`IrregularIruEru`]: Tag::IrregularIruEru
    pub fn is_irregular_iru_eru(&self) -> bool {
        matches!(self, Self::IrregularIruEru)
    }
}

impl Query {
    #[inline]
    pub fn is_valid(&self) -> bool {
        !self.query.is_empty()
    }

    /// Returns true if the query has at least one pos tag
    #[inline]
    pub fn has_part_of_speech_tags(&self) -> bool {
        self.get_part_of_speech_tags().next().is_some()
    }

    /// Returns an iterator over all search type tags
    #[inline]
    pub fn get_search_type_tags(&self) -> impl Iterator<Item = &SearchTypeTag> + '_ {
        self.tags.iter().filter_map(|i| i.as_search_type())
    }

    /// Returns an iterator over all PosSimple tags
    #[inline]
    pub fn get_part_of_speech_tags(&self) -> impl Iterator<Item = &PosSimple> + '_ {
        self.tags.iter().filter_map(|i| i.as_part_of_speech())
    }

    /// Returns an iterator over all Misc tags
    #[inline]
    pub fn get_misc_tags(&self) -> impl Iterator<Item = &Misc> + '_ {
        self.tags.iter().filter_map(|i| i.as_misc())
    }

    /// Returns the result offset by a given page
    #[inline]
    pub fn page_offset(&self, page_size: usize) -> usize {
        query_parser::calc_page_offset(self.page, page_size)
    }

    /// Returns `true` if query has `tag`
    #[inline]
    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.iter().any(|i| *i == tag)
    }

    /// Returns the original_query with search type tags omitted
    #[inline]
    pub fn without_search_type_tags(&self) -> String {
        self.original_query
            .clone()
            .split(' ')
            .into_iter()
            .filter(|i| {
                let is_tag = i.starts_with('#');

                let is_search_type_tag = is_tag
                    .then(|| Tag::parse_from_str(i).map(|i| Tag::is_search_type(&i)))
                    .flatten()
                    .unwrap_or_default();

                // Filter out all search type tags
                (is_tag && !is_search_type_tag) || !is_tag
            })
            .join(" ")
    }

    /// Encodes the query using percent encoding
    pub fn get_query_encoded(&self) -> String {
        utf8_percent_encode(&self.query, QUERY_ENCODE_SET).to_string()
    }

    /// Returns the language with lang override applied
    pub fn get_lang_with_override(&self) -> Language {
        self.language_override.unwrap_or(self.settings.user_lang)
    }

    /// Returns a `RegexSQuery` if the query contains a valid regex
    pub fn as_regex_query(&self) -> Option<RegexSQuery> {
        // Only japanese regex support (for now)
        if self.language != QueryLang::Japanese {
            return None;
        }

        // returns `None` if no regex given, so we don't need to check for that here
        RegexSQuery::new(&self.query)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_jlpt_tag_parsing() {
        assert_eq!(Tag::parse_jlpt_tag("#n4"), Some(Tag::Jlpt(4)));
    }

    #[test]
    fn test_parse_genki_tag_parsing() {
        assert_eq!(Tag::parse_genki_tag("#genki3"), Some(Tag::GenkiLesson(3)));
        assert_eq!(Tag::parse_genki_tag("#genki23"), Some(Tag::GenkiLesson(23)));
    }
}
