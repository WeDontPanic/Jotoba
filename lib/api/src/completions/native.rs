use resources::models::{suggestions::native_words::NativeSuggestion, words::Word};
use utils::binary_search::BinarySearchable;

use super::*;

/// Get suggestions for foreign search input
pub(super) async fn suggestions(query_str: &str) -> Option<Vec<WordPair>> {
    let suggestion_provider = resources::get().suggestions();
    let dict = suggestion_provider.japanese_words()?;
    let word_storage = resources::get().words();

    let mut items: Vec<(WordPair, u32)> = dict
        .search(|e: &NativeSuggestion| search_cmp(e, query_str))
        // Fetch a few more to allow sort-function to give better results
        .take(40)
        .filter_map(|i| {
            word_storage.by_sequence(i.sequence).map(|i| {
                let score = score(i, query_str);
                (i.into(), score)
            })
        })
        .collect();

    items.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    Some(items.into_iter().take(10).map(|i| i.0).collect())
}

#[inline]
fn search_cmp(e: &NativeSuggestion, query_str: &str) -> Ordering {
    if e.text.starts_with(query_str) {
        Ordering::Equal
    } else {
        e.text.as_str().cmp(&query_str)
    }
}

/// Calculate a score for each word result to give better suggestion results
#[inline]
fn score(word: &Word, query_str: &str) -> u32 {
    let mut score = 0;

    if word.reading.get_reading().reading == query_str || word.reading.kana.reading == query_str {
        score += 100;
    }

    if word.is_common() {
        score += 10;
    }

    if let Some(jlpt) = word.get_jlpt_lvl() {
        score += jlpt as u32;
    }

    score
}

impl From<&Word> for WordPair {
    #[inline]
    fn from(word: &Word) -> Self {
        let main_reading = word.get_reading().reading.to_owned();
        if word.reading.kanji.is_some() {
            WordPair {
                secondary: Some(main_reading),
                primary: word.reading.kana.reading.clone(),
            }
        } else {
            WordPair {
                primary: main_reading,
                secondary: None,
            }
        }
    }
}
