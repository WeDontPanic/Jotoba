use std::{cmp::min, collections::BinaryHeap, time::Instant};

use itertools::Itertools;
use japanese::jp_parsing::InputTextParser;
use resources::models::{suggestions::native_words::NativeSuggestion, words::Word};
use search::engine::SearchTask;
use utils::binary_search::BinarySearchable;

use super::super::*;

/// Get suggestions for foreign search input
pub fn suggestions(query_str: &str) -> Option<Vec<WordPair>> {
    let start = Instant::now();

    // parsing
    let query_str_aligned = align_query_str(query_str).unwrap_or_else(|| query_str.to_string());

    let mut items = suggest_words(&[&query_str, &query_str_aligned])?;
    if items.len() <= 4 && !query_str.is_katakana() {
        if let Some(other) = suggest_words(&[&romaji::RomajiExt::to_katakana(query_str)]) {
            items.extend(other);
        }
    }

    println!("suggesting took: {:?}", start.elapsed());

    Some(items.into_iter().map(|i| i.0).unique().take(30).collect())
}

/// Transforms inflections to the main lexeme of the given query
fn align_query_str(query_str: &str) -> Option<String> {
    let in_db = SearchTask::<search::engine::words::native::Engine>::new(query_str).has_term();
    let parser =
        InputTextParser::new(query_str, &japanese::jp_parsing::JA_NL_PARSER, in_db).ok()?;

    if let Some(parsed) = parser.parse() {
        if parsed.items.len() == 1 {
            return Some(parsed.items[0].get_lexeme().to_string());
        }
    }

    None
}

#[derive(PartialEq, Eq)]
struct WordPairOrder((WordPair, u32));

pub(super) fn suggest_words(queries: &[&str]) -> Option<Vec<(WordPair, u32)>> {
    let suggestion_provider = resources::get().suggestions();
    let dict = suggestion_provider.japanese_words()?;
    let word_storage = resources::get().words();

    let mut heap: BinaryHeap<WordPairOrder> = BinaryHeap::with_capacity(50);

    for query in queries {
        let query_romaji = query
            .is_kana()
            .then(|| romaji::RomajiExt::to_romaji(*query));
        heap.extend(
            dict.search(|e: &NativeSuggestion| search_cmp(e, query))
                // Fetch a few more to allow sort-function to give better results
                .take(500)
                .filter_map(|sugg_item| {
                    word_storage.by_sequence(sugg_item.sequence).map(|word| {
                        let score = score(word, &sugg_item, query, &query_romaji);
                        WordPairOrder((word.into(), score))
                    })
                }),
        );
    }

    let res_size = min(heap.len(), 30);
    let mut items = Vec::with_capacity(res_size);
    for _ in 0..res_size {
        items.push(heap.pop()?.0);
    }

    Some(items)
}

/// Calculate a score for each word result to give better suggestion results
fn score(
    word: &Word,
    suggestion_item: &NativeSuggestion,
    query_str: &str,
    query_romaji: &Option<String>,
) -> u32 {
    let word_len = word.get_reading().reading.chars().count();
    let mut score = 0;

    if let Some(jlpt) = word.get_jlpt_lvl() {
        score += (jlpt as u32 + 2) * 10u32;
    }

    if let Some(query_romaji) = query_romaji {
        score += (strsim::jaro(
            &romaji::RomajiExt::to_romaji(word.reading.kana.reading.as_str()),
            &query_romaji,
        ) * 10f64) as u32;
    } else {
        score += (strsim::jaro(&word.reading.get_reading().reading, query_str) * 30f64) as u32;
    }

    if word_len > 1 {
        score += suggestion_item.frequency;
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
