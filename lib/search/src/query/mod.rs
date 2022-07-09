pub mod form;
pub mod parser;
pub mod prefix;
pub mod regex;
pub mod tags;
pub mod user_settings;

pub use form::Form;
pub use tags::Tag;
pub use user_settings::UserSettings;

use self::regex::RegexSQuery;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use std::hash::Hash;
use types::jotoba::{
    languages::Language,
    search::SearchTarget,
    words::{misc::Misc, part_of_speech::PosSimple},
};

const QUERY_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC.add(b'/');

/// A parsed query for a search request
#[derive(Debug, Clone, PartialEq, Default, Hash)]
pub struct Query {
    /// The raw query string from the user without any modifications
    pub raw_query: String,
    /// Parsed query string which will be used to find results
    pub query_str: String,
    /// Where to search {Words,Names,Kanji,Sentences}
    pub target: SearchTarget,
    /// Additional tags eg. #kanji or #jlpt4
    pub tags: Vec<Tag>,
    /// The form of the Query. Eg. KanjiReadingSearch or TagOnly
    pub form: Form,
    /// The language of the passed query string
    pub q_lang: QueryLang,
    /// User settings
    pub settings: UserSettings,
    /// Item offset based on the (current) page
    pub page_offset: usize,
    /// Current page
    pub page: usize,
    /// Word index within a sentence reader search
    pub word_index: usize,
    /// All terms the result has to contain to be shown
    pub must_contain: Vec<String>,
    /// Overwrite the users settings language temporarily
    pub cust_lang: Option<Language>,
}

/// The language of the query content itself
#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
pub enum QueryLang {
    Japanese,
    Foreign,
    Korean,
    #[default]
    Undetected,
}

impl Query {
    /// Returns true if the query has at least one pos tag
    #[inline]
    pub fn has_part_of_speech_tags(&self) -> bool {
        self.get_part_of_speech_tags().next().is_some()
    }

    /// Returns an iterator over all search type tags
    #[inline]
    pub fn get_search_type_tags(&self) -> impl Iterator<Item = &SearchTarget> + '_ {
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
        let (new_query, _) = parser::tags::extract_parse(&self.raw_query, |s| {
            let p = parser::tags::parse(&s);
            if p.is_none() {
                return (None, false);
            }
            (p, p.unwrap().is_search_type())
        });
        new_query
    }

    /// Encodes the parsed query string using percent encoding
    pub fn get_query_encoded(&self) -> String {
        utf8_percent_encode(&self.query_str, QUERY_ENCODE_SET).to_string()
    }

    /// Returns the language with lang override applied
    pub fn get_search_lang(&self) -> Language {
        self.cust_lang.unwrap_or(self.settings.user_lang)
    }

    /// Shortcut for query.settings.user_lang. This does not apply overwritten language. For that use `get_search_lang`
    #[inline]
    pub fn lang(&self) -> Language {
        self.settings.user_lang
    }

    /// Returns a `RegexSQuery` if the query contains a valid regex
    pub fn as_regex_query(&self) -> Option<RegexSQuery> {
        // Only japanese regex support (for now)
        if self.q_lang != QueryLang::Japanese {
            return None;
        }

        // returns `None` if no regex given, so we don't need to check for that here
        RegexSQuery::new(&self.query_str)
    }
}
