pub mod foreign;
pub mod kanji_reading;
pub mod native;
pub mod regex;

use once_cell::sync::Lazy;

/// A Regex matching parentheses and its contents
pub(crate) static REMOVE_PARENTHESES: Lazy<::regex::Regex> =
    Lazy::new(|| ::regex::Regex::new("\\(.*\\)").unwrap());
