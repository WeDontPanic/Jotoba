use std::{cmp::min, collections::BinaryHeap, time::Instant};

use itertools::Itertools;
use japanese::jp_parsing::InputTextParser;
use resources::models::suggestions::native_words::NativeSuggestion;
use search::engine::SearchTask;
use types::jotoba::words::Word;
use utils::binary_search::BinarySearchable;

use super::super::*;

/// Get suggestions for foreign search input
pub fn suggestions(query: &Query, radicals: &[char]) -> Option<Vec<WordPair>> {
    let query_str = query.query.as_str();
    let start = Instant::now();

    // parsing query
    let query_str_aligned = align_query_str(query_str).unwrap_or_else(|| query_str.to_string());

    let mut items = suggest_words(&[&query_str, &query_str_aligned], &radicals)?;
    if items.len() <= 4 && !query_str.is_katakana() {
        if let Some(other) = suggest_words(&[&romaji::RomajiExt::to_katakana(query_str)], &radicals)
        {
            items.extend(other);
        }
    }

    if items.len() < 50 {
        if let Some(aligned) = k_reading_align(query_str) {
            items.extend(aligned);
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

/// Finds suggestions for all kanji componunds which can be built of the given query
fn k_reading_align(query: &str) -> Option<Vec<(WordPair, u32)>> {
    if !query.is_kana() {
        return None;
    }

    let words = resources::get().words();
    let align = storage::K_READING_ALIGN.get().unwrap();

    let res = align
        .get(query)?
        .iter()
        .filter_map(|i| {
            let word = words.by_sequence(*i)?;
            let wp: WordPair = word.into();
            Some((wp, 0))
        })
        .collect::<Vec<_>>();

    Some(res)
}

#[derive(PartialEq, Eq)]
struct WordPairOrder((WordPair, u32));

pub(super) fn suggest_words(
    queries: &[&str],
    filter_radicals: &[char],
) -> Option<Vec<(WordPair, u32)>> {
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
                .filter_map(|sugg_item| {
                    let word = word_storage.by_sequence(sugg_item.sequence)?;

                    // Filter out non radical matching words if radicals are given
                    if !filter_radicals.is_empty()
                        && !word_rad_filter(&query, &word, filter_radicals)
                    {
                        return None;
                    }

                    let score = score(word, &sugg_item, query, &query_romaji);
                    Some(WordPairOrder((word.into(), score)))
                })
                .take(500),
        );
    }

    let res_size = min(heap.len(), 30);
    let mut items = Vec::with_capacity(res_size);
    for _ in 0..res_size {
        items.push(heap.pop()?.0);
    }

    Some(items)
}

fn word_rad_filter(query: &str, word: &Word, radicals: &[char]) -> bool {
    let kanji = match word.reading.kanji.as_ref() {
        Some(k) => &k.reading,
        None => return false,
    };

    let retrieve = resources::get().kanji();

    let query_kanji = query.chars().filter(|i| i.is_kanji()).collect::<Vec<_>>();

    kanji
        .chars()
        // Don't apply on existing kanji
        .filter(|i| !query_kanji.contains(&i))
        .filter_map(|k| k.is_kanji().then(|| retrieve.by_literal(k)).flatten())
        .any(|k| {
            if let Some(k_parts) = &k.parts {
                return is_subset(radicals, &k_parts);
            }
            false
        })
}

/// Returns `true` if `subs` is a subset of `full`
pub fn is_subset<T: PartialEq>(subs: &[T], full: &[T]) -> bool {
    if subs.is_empty() || full.is_empty() || subs.len() > full.len() {
        return false;
    }
    for i in subs {
        if !full.contains(i) {
            return false;
        }
    }
    true
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
