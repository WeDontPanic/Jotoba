pub mod kanji_reading;
pub mod metadata;
pub mod names;
pub mod radical;
pub mod regex;
pub mod relevance;
pub mod sentences;
pub mod storage;
pub mod words;

pub use storage::get;
pub use storage::suggestions::get_suggestions;
