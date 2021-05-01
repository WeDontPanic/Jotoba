use levenshtein::levenshtein;

use crate::{
    japanese::JapaneseExt,
    models::kanji,
    search::{query::Query, SearchMode},
    utils,
};

#[cfg(feature = "tokenizer")]
use super::jp_parsing::WordItem;

use super::result::Word;

pub(super) fn foreign_search_order(word: &Word, search_order: &SearchOrder) -> usize {
    let mut score = 0;
    let reading = word.get_reading();

    if word.is_common() {
        score += 4;
    }

    if let Some(jlpt) = reading.jlpt_lvl {
        score += jlpt as usize;
    }

    score += (calc_likenes(search_order, word, SearchMode::Exact, false) / 10) as usize;
    score += (calc_likenes(search_order, word, SearchMode::Exact, true) / 30) as usize;
    score += (calc_likenes(search_order, word, SearchMode::Variable, false) / 50) as usize;
    score += (calc_likenes(search_order, word, SearchMode::Variable, true) / 80) as usize;

    score
}

pub fn get_query_pos_in_gloss(
    search_order: &SearchOrder,
    this: &Word,
    s_mode: SearchMode,
    ign_case: bool,
) -> Option<usize> {
    let items = this.get_senses();

    for lang_senes in items.iter() {
        let mut pos = 0;
        for sense in lang_senes {
            for gloss in sense.glosses.iter() {
                if s_mode.str_eq(&gloss.gloss, &search_order.query.query, ign_case) {
                    return Some(pos);
                }

                pos += 1;
            }
        }
    }

    None
}

/// Returns a value from 1 to 100 based on importance
/// an item inside a result
fn calc_importance(pos: usize, total: usize) -> usize {
    (pos * 100) / total
}

pub fn calc_likenes(
    search_order: &SearchOrder,
    this: &Word,
    s_mode: SearchMode,
    ign_case: bool,
) -> u8 {
    let n: usize = this.senses.iter().map(|i| i.glosses.iter().count()).sum();
    let pos = get_query_pos_in_gloss(search_order, this, s_mode, ign_case);
    if pos.is_none() {
        return 0;
    }
    100 - calc_importance(pos.unwrap(), n) as u8
}

/// Search order for words searched by japanese meaning/kanji/reading
pub(super) fn native_search_order(word: &Word, search_order: &SearchOrder) -> usize {
    #[cfg(feature = "tokenizer")]
    let morpheme = search_order.morpheme;

    let reading = word.get_reading();
    let kana_reading = word.reading.kana.as_ref().unwrap();
    // Original query
    let query = search_order.query;
    // The original query text
    let query_str = &query.query;

    let mut score = 0;

    if word.is_common() {
        score += 8;
    }

    if reading.reading == *query_str || kana_reading.reading == *query_str {
        score += 30;

        // Show kana only readings on top if they match with query
        if kana_reading.reading == *query_str && word.reading.kanji.is_none() {
            score += 20;
        }
    }

    if let Some(jlpt) = reading.jlpt_lvl {
        score += jlpt as usize;
    }

    #[cfg(feature = "tokenizer")]
    if let Some(morpheme) = morpheme {
        let lexeme = morpheme.get_lexeme();
        if reading.reading == lexeme || kana_reading.reading == lexeme {
            score += 15;
        }

        // Show kana only readings on top if they match with lexeme
        if kana_reading.reading == lexeme && word.reading.kanji.is_none() {
            score += 30;
        }
    }

    score += word
        .priorities
        .as_ref()
        .map(|i| i.len())
        .unwrap_or_default()
        * 2;

    score
}

pub(super) fn kanji_reading_search(word: &Word, search_order: &SearchOrder) -> usize {
    let mut score = 0;
    // This function should only be called for kanji reading search queries!
    debug_assert!(search_order.query.form.as_kanji_reading().is_some());
    let kanji_reading = search_order.query.form.as_kanji_reading().unwrap();
    let formatted_reading = kanji::format_reading(&kanji_reading.reading);
    let kana_reading = &word.reading.kana.as_ref().unwrap().reading;

    if formatted_reading.is_hiragana() {
        // Kun reading
        if *kana_reading == formatted_reading
            // Don't show direct readings if the kanji reading is a suffix/prefix
            && !kanji_reading.reading.starts_with("-")
            && !kanji_reading.reading.ends_with("-")
        {
            score += 20;
        }
    } else {
        if kana_reading.to_hiragana() == formatted_reading.to_hiragana() {
            score += 100;
        } else if kana_reading
            .to_hiragana()
            .contains(&formatted_reading.to_hiragana())
        {
            // On reading
            score +=
                (20 - levenshtein(
                    &kana_reading.to_hiragana(),
                    &formatted_reading.to_hiragana(),
                )) * 2;
        }
    }

    if word.is_common() {
        score += 8;
    }

    if let Some(jlpt) = word.get_reading().jlpt_lvl {
        score += jlpt as usize;
    }

    score
}

#[cfg(feature = "tokenizer")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SearchOrder<'a, 'parser> {
    query: &'a Query,
    morpheme: &'a Option<WordItem<'parser, 'a>>,
}

#[cfg(not(feature = "tokenizer"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SearchOrder<'a> {
    query: &'a Query,
}

#[cfg(feature = "tokenizer")]
impl<'a, 'parser> SearchOrder<'a, 'parser> {
    pub fn new(query: &'a Query, morpheme: &'a Option<WordItem<'parser, 'a>>) -> Self {
        SearchOrder { query, morpheme }
    }

    pub fn sort<U, T>(&self, vec: &mut Vec<U>, order_fn: T)
    where
        T: Fn(&U, &SearchOrder) -> usize,
    {
        vec.sort_by(|a, b| utils::invert_ordering(order_fn(a, &self).cmp(&order_fn(b, &self))))
    }
}

#[cfg(not(feature = "tokenizer"))]
impl<'a, 'parser> SearchOrder<'a> {
    pub fn new(query: &'a Query) -> Self {
        SearchOrder { query }
    }

    pub fn sort<U, T>(&self, vec: &mut Vec<U>, order_fn: T)
    where
        T: Fn(&U, &SearchOrder) -> usize,
    {
        vec.sort_by(|a, b| utils::invert_ordering(order_fn(a, &self).cmp(&order_fn(b, &self))))
    }
}
