pub mod form;
pub mod parser;
pub mod regex;
pub mod tags;
pub mod user_settings;

pub use form::Form;
pub use tags::Tag;
pub use user_settings::UserSettings;

use self::{regex::RegexSQuery, tags::SearchTypeTag};
use itertools::Itertools;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::hash::Hash;
use types::jotoba::{
    languages::Language,
    search::QueryType,
    words::{misc::Misc, part_of_speech::PosSimple},
};

const QUERY_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC.add(b'/');

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

/// The language of the query
#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum QueryLang {
    Japanese,
    Foreign,
    Korean,
    Undetected,
}

impl Default for QueryLang {
    #[inline]
    fn default() -> Self {
        Self::Undetected
    }
}

impl Query {
    /// Returns true if the query is valid
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
        parser::calc_page_offset(self.page, page_size)
    }

    /// Returns `true` if query has `tag`
    #[inline]
    pub fn has_tag(&self, tag: Tag) -> bool {
        self.tags.iter().any(|i| *i == tag)
    }

    /// Adds `n` pages to the query
    pub fn add_page(&mut self, n: usize) {
        self.page = (self.page + n).min(100);
        self.page_offset += (self.settings.page_size as usize) * n;
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
