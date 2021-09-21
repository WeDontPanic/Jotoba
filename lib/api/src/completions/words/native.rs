use std::{cmp::min, collections::BinaryHeap, time::Instant};

use resources::models::{suggestions::native_words::NativeSuggestion, words::Word};
use utils::binary_search::BinarySearchable;

use super::super::*;

/// Get suggestions for foreign search input
pub async fn suggestions(query_str: &str) -> Option<Vec<WordPair>> {
    let start = Instant::now();
    let items = suggest_words(query_str)?;
    println!("suggesting took: {:?}", start.elapsed());

    let mut results: Vec<_> = items.into_iter().take(10).map(|i| i.0).collect();
    results.dedup();

    Some(results)
}

#[derive(PartialEq, Eq)]
struct WordPairOrder((WordPair, u32));

fn suggest_words(query_str: &str) -> Option<Vec<(WordPair, u32)>> {
    let query_romaji = query_str
        .is_kana()
        .then(|| romaji::RomajiExt::to_romaji(query_str));

    let suggestion_provider = resources::get().suggestions();
    let dict = suggestion_provider.japanese_words()?;
    let word_storage = resources::get().words();

    let mut heap: BinaryHeap<WordPairOrder> = BinaryHeap::with_capacity(50);

    heap.extend(
        dict.search(|e: &NativeSuggestion| search_cmp(e, query_str))
            // Fetch a few more to allow sort-function to give better results
            .take(50)
            .filter_map(|sugg_item| {
                word_storage.by_sequence(sugg_item.sequence).map(|word| {
                    let score = score(word, query_str, &query_romaji);
                    WordPairOrder((word.into(), score))
                })
            }),
    );

    let res_size = min(heap.len(), 10);
    let mut items = Vec::with_capacity(res_size);
    for _ in 0..res_size {
        items.push(heap.pop()?.0);
    }

    Some(items)
}

/// Calculate a score for each word result to give better suggestion results
fn score(word: &Word, query_str: &str, query_romaji: &Option<String>) -> u32 {
    let mut score = 0;

    if word.is_common() {
        score += 10;
    }

    if let Some(jlpt) = word.get_jlpt_lvl() {
        score += (jlpt as u32 + 2) * 10u32;
    }

    if let Some(query_romaji) = query_romaji {
        score += (strsim::jaro(
            &romaji::RomajiExt::to_romaji(word.reading.kana.reading.as_str()),
            &query_romaji,
        ) * 100f64) as u32;
    } else {
        score += (strsim::jaro(&word.reading.get_reading().reading, query_str) * 70f64) as u32;
    }

    score
}

#[inline]
fn search_cmp(e: &NativeSuggestion, query_str: &str) -> Ordering {
    if e.text.starts_with(query_str) {
        Ordering::Equal
    } else {
        e.text.as_str().cmp(&query_str)
    }
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

impl Ord for WordPairOrder {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0 .1.cmp(&other.0 .1)
    }
}

impl PartialOrd for WordPairOrder {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0 .1.cmp(&other.0 .1))
    }
}
