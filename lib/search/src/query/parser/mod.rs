pub(crate) mod prefix;
pub(crate) mod tags;

use super::{prefix::SearchPrefix, regex::RegexSQuery, Form, Query, QueryLang, Tag, UserSettings};
use itertools::Itertools;
use japanese::JapaneseExt;
use std::cmp::Ordering;
use types::jotoba::{
    kanji, languages::Language as ContentLanguage, search::SearchTarget,
    words::part_of_speech::PosSimple,
};

/// Max amount of characters a query is allowed to have
pub const MAX_QUERY_LEN: usize = 400;

/// Amount of characters (in percent) that have to be Japanese characters
/// in order to handle the input as Japanese text
pub const JAPANESE_THRESHOLD: usize = 40;

/// Represents a query
pub struct QueryParser {
    q_type: SearchTarget,
    raw_query: String,
    tags: Vec<Tag>,
    user_settings: UserSettings,
    page_offset: usize,
    page: usize,
    word_index: usize,
    language_override: Option<ContentLanguage>,
}

impl QueryParser {
    /// Create a new QueryParser
    pub fn new(
        raw_query: String,
        query_type: SearchTarget,
        user_settings: UserSettings,
    ) -> QueryParser {
        QueryParser {
            q_type: query_type,
            raw_query,
            tags: vec![],
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

        let (new_query, tags) = Self::partition_tags_query(&stripped);
        let query: String = new_query
            .trim()
            .chars()
            .into_iter()
            .take(MAX_QUERY_LEN)
            .collect();

        // Don't allow empty queries
        if query.is_empty() && !self.tags.iter().any(|i| i.is_empty_allowed()) {
            println!("empty");
            return None;
        }

        let parse_japanese = self.need_jp_parsing();

        Some(Query {
            q_lang: parse_language(&query),
            target: self.get_search_target(),
            form: self.parse_form(&query),
            tags,
            query_str: query,
            raw_query: self.raw_query,
            settings: self.user_settings,
            page_offset: self.page_offset,
            page: self.page,
            word_index: self.word_index,
            parse_japanese,
            cust_lang: self.language_override,
        })
    }

    // Split the query string into tags and the actual query
    fn partition_tags_query(query_str: &str) -> (String, Vec<Tag>) {
        tags::extract_parse(query_str, |t_s| {
            let parsed = tags::parse(&t_s.to_lowercase());
            (parsed, true)
        })
    }

    fn need_jp_parsing(&self) -> bool {
        let mod_tags = self
            .tags
            .iter()
            .filter(|i| i.is_search_type() && *i.as_search_type().unwrap() == SearchTarget::Words)
            .collect_vec();

        self.tags.is_empty()
            || utils::same_elements(&mod_tags, &[&Tag::PartOfSpeech(PosSimple::Verb)])
    }

    /// Parses the QueryType based on the user selection and tags
    #[inline]
    fn get_search_target(&self) -> SearchTarget {
        self.tags
            .iter()
            .filter_map(|i| i.as_search_type())
            .copied()
            .next()
            .unwrap_or(self.q_type)
    }

    fn parse_form(&self, query: &str) -> Form {
        // Tag only search
        if query.is_empty() && self.tags.iter().any(|i| i.is_empty_allowed()) {
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
        if utils::real_string_len(query) >= 3 && query.contains(' ') {
            let split: Vec<_> = query.split(' ').collect();

            let kanji_literal = split[0].trim();

            if kanji_literal.trim().is_kanji()
                && format_kanji_reading(split[1]).is_japanese()
                // don't allow queries like '音楽 おと'
                && utils::real_string_len(kanji_literal) == 1
            {
                // Kanji detected
                return Some(kanji::reading::ReadingSearch {
                    literal: split[0].chars().next().unwrap(),
                    reading: split[1].to_string(),
                });
            }
        }

        None
    }
}

/// Returns a number 0-100 of japanese character ratio
fn get_jp_part(inp: &str) -> usize {
    let mut total = 0;
    let mut japanese = 0;
    for c in inp.chars() {
        total += 1;
        if c.is_japanese() {
            japanese += 1;
        }
    }

    ((japanese as f32 / total as f32) * 100f32) as usize
}

/// Tries to determine between Japanese/Non japnaese
pub fn parse_language(query: &str) -> QueryLang {
    let query = strip_regex(query).unwrap_or_else(|| query.to_string());
    if utils::korean::is_hangul_str(&query) {
        return QueryLang::Korean;
    }

    let query = format_kanji_reading(&query);

    match get_jp_part(&query).cmp(&JAPANESE_THRESHOLD) {
        Ordering::Equal => QueryLang::Undetected,
        Ordering::Less => QueryLang::Foreign,
        Ordering::Greater => QueryLang::Japanese,
    }
}

#[inline]
pub fn format_kanji_reading(s: &str) -> String {
    s.replace('.', "").replace('-', "").replace(' ', "")
}

pub fn calc_page_offset(page: usize, page_size: usize) -> usize {
    page.saturating_sub(1) * page_size
}

/// Removes regex parts from a query. Returns `None` if `query` does not contain regex symbols
fn strip_regex(query: &str) -> Option<String> {
    Some(RegexSQuery::new(query)?.get_chars().into_iter().collect())
}
