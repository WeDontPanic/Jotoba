use std::cmp::Ordering;

use itertools::Itertools;
use localization::{language::Language, traits::Translatable, TranslationDict};
use serde::Deserialize;

use japanese::JapaneseExt;
use resources::{models::kanji, parse::jmdict::part_of_speech::PosSimple};

use super::query::{Form, Query, QueryLang, SearchTypeTag, Tag, UserSettings};

/// Represents a query
pub struct QueryParser {
    q_type: QueryType,
    query: String,
    original_query: String,
    tags: Vec<Tag>,
    user_settings: UserSettings,
    page_offset: usize,
    page: usize,
    word_index: usize,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Hash)]
pub enum QueryType {
    #[serde(rename = "1")]
    Kanji,
    #[serde(rename = "2")]
    Sentences,
    #[serde(rename = "3")]
    Names,
    #[serde(rename = "0", other)]
    Words,
}

impl QueryType {
    /// Iterate over all query types
    pub fn iterate() -> impl Iterator<Item = Self> {
        vec![Self::Kanji, Self::Sentences, Self::Names, Self::Words].into_iter()
    }

    pub fn get_translated<'a>(
        &self,
        dict: &'a TranslationDict,
        language: Option<Language>,
    ) -> &'a str {
        dict.gettext(self.get_id(), language)
    }

    #[inline]
    pub fn get_type_id(&self) -> u8 {
        match self {
            QueryType::Kanji => 1,
            QueryType::Sentences => 2,
            QueryType::Names => 3,
            QueryType::Words => 0,
        }
    }
}

impl Translatable for QueryType {
    #[inline]
    fn get_id(&self) -> &'static str {
        match self {
            QueryType::Kanji => "Kanji",
            QueryType::Sentences => "Sentences",
            QueryType::Names => "Names",
            QueryType::Words => "Words",
        }
    }
}

impl Default for QueryType {
    #[inline]
    fn default() -> Self {
        Self::Words
    }
}

impl QueryParser {
    pub fn new(
        query: String,
        q_type: QueryType,
        user_settings: UserSettings,
        page: usize,
        word_index: usize,
        trim: bool,
    ) -> QueryParser {
        // Split query into the actual query and possibly available tags
        let (parsed_query, tags) = Self::partition_tags_query(&query, trim);
        let parsed_query: String = Self::format_query(parsed_query, trim)
            .chars()
            .into_iter()
            .take(80)
            .collect();

        // Pages start at 1. First offset has to be 0
        let page_offset = (page.saturating_sub(1)) * user_settings.items_per_page as usize;
        println!("page_offset: {}", page_offset);

        QueryParser {
            q_type,
            query: parsed_query,
            original_query: query,
            tags,
            user_settings,
            page_offset,
            page,
            word_index,
        }
    }

    // Split the query string into tags and the actual query
    fn partition_tags_query(query_str: &str, trim: bool) -> (String, Vec<Tag>) {
        // TODO don't split by space to allow queries like: '<KANJI>#kanji'
        let (tags, query): (Vec<&str>, Vec<&str>) =
            query_str.split(' ').partition(|i| i.starts_with('#'));

        let mut query = query.join(" ").trim().to_string();
        let tags = tags
            .into_iter()
            .filter_map(|i| Tag::parse_from_str(&i.to_lowercase()))
            .collect();

        // TODO this is ugly but works for our needs
        if !trim && query_str.ends_with(' ') {
            query.push(' ');
        }

        (query, tags)
    }

    /// Parses a user query into Query
    pub fn parse(self) -> Option<Query> {
        // Don't allow empty queries
        if self.query.is_empty() && !self.tags.iter().any(|i| i.is_empty_allowed()) {
            println!("empty");
            return None;
        }

        let parse_japanese = self.need_jp_parsing();

        Some(Query {
            language: parse_language(&self.query),
            type_: self.parse_query_type(),
            form: self.parse_form(),
            tags: self.tags,
            query: self.query,
            original_query: self.original_query,
            settings: self.user_settings,
            page_offset: self.page_offset,
            page: self.page,
            word_index: self.word_index,
            parse_japanese,
        })
    }

    fn need_jp_parsing(&self) -> bool {
        let mod_tags = self
            .tags
            .iter()
            .filter(|i| i.is_search_type() && *i.as_search_type().unwrap() == SearchTypeTag::Word)
            .collect_vec();

        self.tags.is_empty()
            || utils::same_elements(&mod_tags, &[&Tag::PartOfSpeech(PosSimple::Verb)])
    }

    /// Formats the query
    fn format_query(query: String, trim: bool) -> String {
        if trim { query.trim().to_owned() } else { query }.replace("%", "")
    }

    /// Parses the QueryType based on the user selection and tags
    fn parse_query_type(&self) -> QueryType {
        if self.tags.contains(&Tag::SearchType(SearchTypeTag::Kanji)) {
            QueryType::Kanji
        } else if self.tags.contains(&Tag::SearchType(SearchTypeTag::Word)) {
            QueryType::Words
        } else if self
            .tags
            .contains(&Tag::SearchType(SearchTypeTag::Sentence))
        {
            QueryType::Sentences
        } else if self.tags.contains(&Tag::SearchType(SearchTypeTag::Name)) {
            QueryType::Names
        } else {
            // No QueryType-Tag provided use
            // drop-down selection
            self.q_type
        }
    }

    fn parse_form(&self) -> Form {
        let query = &self.query;

        // Tag only search
        if query.is_empty() && self.tags.iter().any(|i| i.is_empty_allowed()) {
            return Form::TagOnly;
        }

        // Detect a kanji reading query
        if let Some(kr) = self.parse_kanji_reading() {
            return Form::KanjiReading(kr);
        }

        // Japanese only input
        if query.is_japanese() {
            return Form::SingleWord;
        }

        // Non Japanese input
        if !query.has_japanese() {
            // Assuming every other supported language is
            // not retarded and splits its word with spaces
            return if self.query.contains(' ') {
                Form::MultiWords
            } else {
                Form::SingleWord
            };
        }

        Form::Undetected
    }

    /// Returns Some(KanjiReading) if the query is a kanji reading query
    fn parse_kanji_reading(&self) -> Option<kanji::Reading> {
        // Format of kanji query: '<Kanji> <reading>'
        if utils::real_string_len(&self.query) >= 3 && self.query.contains(' ') {
            let split: Vec<_> = self.query.split(' ').collect();

            if split[0].trim().is_kanji() && format_kanji_reading(split[1]).is_japanese() {
                // Kanji detected
                return Some(kanji::Reading {
                    literal: split[0].chars().next().unwrap(),
                    reading: split[1].to_string(),
                });
            }
        }

        None
    }
}

/// Returns a number 0-100 of japanese character ratio
fn get_jp_part(inp: &str) -> u8 {
    let mut total = 0;
    let mut japanese = 0;
    for c in inp.chars() {
        total += 1;
        if c.is_japanese() {
            japanese += 1;
        }
    }

    ((japanese as f32 / total as f32) * 100f32) as u8
}

/// Tries to determine between Japanese/Non japnaese
pub fn parse_language(query: &str) -> QueryLang {
    let query = format_kanji_reading(query);

    // how many percent of the characters have to be japanese in order to rank a text as japanese text
    let threshold = 40;

    match get_jp_part(&query).cmp(&threshold) {
        Ordering::Equal => QueryLang::Undetected,
        Ordering::Less => QueryLang::Foreign,
        Ordering::Greater => QueryLang::Japanese,
    }
}

#[inline]
pub fn format_kanji_reading(s: &str) -> String {
    s.replace('.', "").replace('-', "").replace(' ', "")
}
