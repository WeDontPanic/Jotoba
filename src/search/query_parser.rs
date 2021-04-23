use serde::Deserialize;

use crate::{japanese::JapaneseExt, utils};

use super::query::{Form, KanjiReading, Query, QueryLang, Tag};

/// Represents a query
pub struct QueryParser {
    q_type: QueryType,
    query: String,
    tags: Vec<Tag>,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
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

impl Default for QueryType {
    fn default() -> Self {
        Self::Words
    }
}

impl QueryParser {
    pub fn new(query: String, q_type: QueryType) -> QueryParser {
        // Split query into the actual query and possibly available tags
        let (query, tags) = Self::partition_tags_query(&query);
        let query = Self::format_query(query);

        QueryParser {
            q_type,
            query,
            tags,
        }
    }

    // Split the query string into tags and the actual query
    fn partition_tags_query(query: &str) -> (String, Vec<Tag>) {
        // TODO don't split by space to allow queries like: '<KANJI>#kanji'
        let (tags, query): (Vec<&str>, Vec<&str>) =
            query.split(" ").partition(|i| i.starts_with("#"));

        let query = query.join(" ").trim().to_string();
        let tags = tags.into_iter().filter_map(|i| Tag::from_str(i)).collect();

        (query, tags)
    }

    /// Parses a user query into Query
    pub fn parse(self) -> Option<Query> {
        // Don't allow empty queries
        if self.query.is_empty() {
            return None;
        }

        Some(Query {
            language: self.parse_language(&self.query),
            form: self.parse_form(),
            tags: self.tags,
            items: self.query,
            type_: self.q_type,
        })
    }

    /// Formats the query
    fn format_query(query: String) -> String {
        query.trim().to_string()
    }

    fn parse_form(&self) -> Form {
        let query = &self.query;

        // Japanese only input
        if query.is_japanese() {
            // TODO actually dected if there are multiple words
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

        // Detect a kanji reading query
        if let Some(kr) = self.parse_kanji_reading() {
            return Form::KanjiReading(kr);
        }

        Form::Undetected
    }

    /// Returns Some(KanjiReading) if the query is a kanji reading query
    fn parse_kanji_reading(&self) -> Option<KanjiReading> {
        // Format of kanji query: '<Kanji> <reading>'
        if utils::real_string_len(&self.query) >= 3 && self.query.contains(' ') {
            let split: Vec<_> = self.query.split(' ').collect();

            if split[0].trim().is_kanji() && split[1].is_japanese() {
                // Kanji detected
                return Some(KanjiReading {
                    literal: split[0].chars().next().unwrap(),
                    reading: split[1].to_string(),
                });
            }
        }

        None
    }

    // Tries to determine between Japanese/Non japnaese
    fn parse_language(&self, query: &str) -> QueryLang {
        if query.is_japanese() {
            QueryLang::Japanese
        } else if !query.has_japanese() {
            QueryLang::Foreign
        } else {
            QueryLang::Undetected
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_partition_tags() {
        let query = QueryParser::new("ashi #kanji".to_string(), QueryType::Words);

        assert_eq!(query.query, "ashi".to_string());
        assert_eq!(query.tags, vec![Tag::Kanji]);

        let query = QueryParser::new("#kanji #adjective #ee #  #ea".to_string(), QueryType::Words);

        assert_eq!(query.query, "".to_string());
        assert_eq!(query.tags, vec![Tag::Kanji, Tag::Adjective]);
    }

    #[test]
    fn test_kanji_reading_detection() {
        let query = QueryParser::new("気 ケ".to_string(), QueryType::Words).parse();
        assert!(query.is_some());
        let query = query.unwrap();
        assert_eq!(
            query.form,
            Form::KanjiReading(KanjiReading {
                literal: '気',
                reading: "ケ".to_string(),
            })
        );
    }
}
