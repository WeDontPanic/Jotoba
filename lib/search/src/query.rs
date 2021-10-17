use std::{
    hash::{Hash, Hasher},
    str::FromStr,
};

use super::query_parser::QueryType;

use itertools::Itertools;
use resources::{
    models::kanji,
    parse::jmdict::{languages::Language, misc::Misc, part_of_speech::PosSimple},
};

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
}

/// In-cookie saved personalized settings
#[derive(Debug, Clone, Copy)]
pub struct UserSettings {
    pub user_lang: Language,
    pub page_lang: localization::language::Language,
    pub show_english: bool,
    pub english_on_top: bool,
    pub cookies_enabled: bool,
    pub items_per_page: u32,
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
            items_per_page: 10,
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
    KanjiReading(kanji::Reading),
    /// Tag only. Implies query string to be empty
    TagOnly,
    /// Form was not recognized
    Undetected,
}

impl Form {
    #[inline]
    pub fn as_kanji_reading(&self) -> Option<&kanji::Reading> {
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
        #[allow(irrefutable_let_patterns)]
        if let Some(tag) = Self::parse_jlpt_tag(s) {
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
    pub fn is_empty_allowed(&self) -> bool {
        self.is_jlpt()
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_jlpt_tag_parsing() {
        assert_eq!(Tag::parse_jlpt_tag("#n4"), Some(Tag::Jlpt(4)));
    }
}
