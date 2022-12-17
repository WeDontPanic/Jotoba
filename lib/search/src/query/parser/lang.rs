use crate::query::{regex::RegexSQuery, QueryLang};
use jp_utils::JapaneseExt;
use std::cmp::Ordering;

use super::JAPANESE_THRESHOLD;

/// Tries to determine between Japanese/Non japnaese
pub fn parse(query: &str) -> QueryLang {
    let query = strip_regex(query).unwrap_or_else(|| query.to_string());
    if utils::korean::is_hangul_str(&query) {
        return QueryLang::Korean;
    }

    match get_jp_part(&query).cmp(&JAPANESE_THRESHOLD) {
        Ordering::Equal => QueryLang::Undetected,
        Ordering::Less => QueryLang::Foreign,
        Ordering::Greater => QueryLang::Japanese,
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

/// Removes regex parts from a query. Returns `None` if `query` does not contain regex symbols
fn strip_regex(query: &str) -> Option<String> {
    Some(RegexSQuery::new(query)?.get_chars().into_iter().collect())
}
