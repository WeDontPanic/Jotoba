//! A regex like search query to search in words with wildcards
//!
//! # Example
//!
//! "宇宙*行士" => "宇宙飛行士"
//!
//! # Supported syntax
//! `*` - Allows 0-n other characters
//! `?` - Allows 1 other characters
//!
//! # Note
//! All queries containing (custom)regex syntax will be handled as full-word matches. In other words if
//! a query contains regex syntax, only full words will be matched. If words should also be open to
//! an end (eg. right variable) then a regex charecter has to be placed at the end as well

use regex::Regex;

/// All characters treated as regex characters
pub const REGEX_CHARS: &[char] = &['*', '?', '?'];

/// Regex Search query. Can be used to match words
#[derive(Clone, Debug)]
pub struct RegexSQuery {
    query: String,
    regex: Regex,
}

impl RegexSQuery {
    /// Create a new regex query. Returns `None` if invalid or no regex given
    pub fn new(query: &str) -> Option<Self> {
        let query = adjust_regex(query);

        if !Self::is_regex(&query) {
            return None;
        }

        let regex = Regex::new(&Self::convert_regex(&query)).ok()?;
        Some(RegexSQuery { query, regex })
    }

    /// Returns `true` if a word matches the regex query
    #[inline]
    pub fn matches(&self, word: &str) -> bool {
        self.regex.is_match(word)
    }

    /// Returns the query-information holding characters from the query. In other words those, who
    /// don't represent regex syntax
    pub fn get_chars(&self) -> Vec<char> {
        let mut out = Vec::with_capacity(self.query.len());
        for c in self.query.chars() {
            if !REGEX_CHARS.contains(&c) {
                out.push(c);
            }
        }
        out
    }

    /// Returns a real regex expression which will be used to match words
    fn convert_regex(query: &str) -> String {
        let mut out = String::with_capacity(query.len() + 2);
        out.push('^');
        out.push_str(
            &query
                .replace('*', ".*")
                .replace('?', ".{1}")
                .replace('+', ".{1}"),
        );
        if !out.ends_with('$') {
            out.push('$');
        }
        out
    }

    /// Returns `true` if query can be interpreted as regex query
    #[inline]
    fn is_regex(query: &str) -> bool {
        let query = adjust_regex(query);
        query.contains('*') || query.contains('+') || query.contains('?')
    }

    /// Get a reference to the regex squery's query.
    pub fn query(&self) -> &str {
        self.query.as_ref()
    }
}

/// Adjusts the query to a consistent format
#[inline]
fn adjust_regex(query: &str) -> String {
    query
        .replace('＊', "*")
        .replace('＋', "+")
        .replace('？', "?")
}
