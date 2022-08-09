pub mod hashtag;
pub mod kanji;
pub mod metadata;
pub mod names;
pub mod radical;
pub mod regex;
pub mod relevance;
pub mod sentences;
pub mod storage;
pub mod words;

pub use storage::{get, suggestions::get_suggestions};
