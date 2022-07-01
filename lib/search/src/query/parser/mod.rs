pub mod lang;
pub(crate) mod prefix;
pub mod req_terms;
pub(crate) mod tags;

use super::{prefix::SearchPrefix, Form, Query, Tag, UserSettings};
use japanese::JapaneseExt;
use types::jotoba::{kanji, languages::Language as ContentLanguage, search::SearchTarget};

/// Max amount of characters a query is allowed to have
pub const MAX_QUERY_LEN: usize = 400;

/// Amount of characters (in percent) that have to be Japanese characters
/// in order to handle the input as Japanese text
pub const JAPANESE_THRESHOLD: usize = 40;

/// Represents a query
pub struct QueryParser {
    /// Where to search {Words,Names,Kanji,Sentences}
    q_type: SearchTarget,
    /// The unmodified query from the search-input
    raw_query: String,
    /// Users settings
    user_settings: UserSettings,
    /// Item offset based on the picked page
    page_offset: usize,
    /// Current page
    page: usize,
    /// Word index for the sentence reader
    word_index: usize,
    /// Overwrite the users settings language
    language_override: Option<ContentLanguage>,
}

impl QueryParser {
    /// Create a new QueryParser
    pub fn new(
        raw_query: String,
        q_type: SearchTarget,
        user_settings: UserSettings,
    ) -> QueryParser {
        QueryParser {
            raw_query,
            q_type,
            user_settings,
            page_offset: 0,
            page: 0,
            word_index: 0,
            language_override: None,
        }
    }

    #[inline]
    pub fn with_lang_overwrite(mut self, lang: ContentLanguage) -> Self {
        self.language_override = Some(lang);
        self
    }

    #[inline]
    pub fn with_word_index(mut self, word_index: usize) -> Self {
        self.word_index = word_index;
        self
    }

    #[inline]
    pub fn with_page(mut self, page: usize) -> Self {
        self.page = page;
        self.page_offset = calc_page_offset(page, self.user_settings.page_size as usize);
        self
    }

    /// Parses a user query into Query
    pub fn parse(mut self) -> Option<Query> {
        let (stripped, s_prefix) = prefix::parse_prefix(&self.raw_query);
        if let Some(SearchPrefix::LangOverwrite(r#lang_overwrite)) = s_prefix {
            self.language_override = Some(lang_overwrite);
        }

        let (new_query, tags) = Self::extract_tags(&stripped);
        let (new_query, must_contain) = req_terms::parse(&new_query);
        let query_str: String = new_query
            .trim()
            .chars()
            .into_iter()
            .take(MAX_QUERY_LEN)
            .collect();

        // Don't allow empty queries
        if query_str.is_empty() && !tags.iter().any(|i| i.is_producer()) {
            return None;
        }

        let q_lang = lang::parse(&query_str);
        let target = self.get_search_target(&tags);
        let form = self.parse_form(&query_str, &tags, s_prefix);

        Some(Query {
            q_lang,
            target,
            form,
            tags,
            query_str,
            raw_query: self.raw_query,
            settings: self.user_settings,
            page_offset: self.page_offset,
            page: self.page,
            word_index: self.word_index,
            cust_lang: self.language_override,
            must_contain,
        })
    }

    // Extracts all tags from `query_str` and returns a new String along with the extracted tags
    #[inline]
    fn extract_tags(query_str: &str) -> (String, Vec<Tag>) {
        tags::extract_parse(query_str, |t_s| {
            let s = t_s.to_lowercase();
            (tags::parse(&s), true)
        })
    }

    /// Parses the QueryType based on the user selection and tags
    #[inline]
    fn get_search_target(&self, tags: &[Tag]) -> SearchTarget {
        tags.iter()
            .filter_map(|i| i.as_search_type())
            .copied()
            .next()
            .unwrap_or(self.q_type)
    }

    fn parse_form(&self, query: &str, tags: &[Tag], s_prefix: Option<SearchPrefix>) -> Form {
        // Sequence search
        if let Some(SearchPrefix::BySequence(r#seq)) = s_prefix {
            return Form::Sequence(seq);
        }

        // Tag only search
        if query.is_empty() && tags.iter().any(|i| i.is_producer()) {
            return Form::TagOnly;
        }

        // Detect a kanji reading query
        if let Some(kr) = self.parse_kanji_reading(query) {
            return Form::KanjiReading(kr);
        }

        // Japanese only input
        if query.is_japanese() {
            return Form::SingleWord;
        }

        // Non Japanese input
        if !query.has_japanese() {
            // Assuming every other supported language is
            // not as retarded as Japanese and actually uses spaces INSTEAD OF FUCKING 2000 CHARACTERS FFS
            return if query.contains(' ') {
                Form::MultiWords
            } else {
                Form::SingleWord
            };
        }

        Form::Undetected
    }

    /// Returns Some(KanjiReading) if the query is a kanji reading query
    fn parse_kanji_reading(&self, query: &str) -> Option<kanji::reading::ReadingSearch> {
        // Format of kanji query: '<Kanji> <reading>'
        if utils::real_string_len(query) < 3 || !query.contains(' ') {
            return None;
        }

        let split: Vec<_> = query.split(' ').collect();

        let kanji_lit = split[0].trim();

        if kanji_lit.is_kanji()
                && format_kanji_reading(split[1]).is_japanese()
                // don't allow queries like '音楽 おと'
                && utils::real_string_len(kanji_lit) == 1
        {
            // Kanji detected
            return Some(kanji::reading::ReadingSearch {
                literal: split[0].chars().next().unwrap(),
                reading: split[1].to_string(),
            });
        }

        None
    }
}

#[inline]
pub fn format_kanji_reading(s: &str) -> String {
    s.replace('.', "").replace('-', "").replace(' ', "")
}

pub fn calc_page_offset(page: usize, page_size: usize) -> usize {
    page.saturating_sub(1) * page_size
}
