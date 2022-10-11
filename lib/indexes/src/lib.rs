pub mod hashtag;
pub mod kanji;
pub mod names;
pub mod ng_freq;
pub mod radical;
pub mod regex;
pub mod sentences;
pub mod storage;
pub mod term_freq;
pub mod words;

pub use storage::{get, suggestions::get_suggestions};
