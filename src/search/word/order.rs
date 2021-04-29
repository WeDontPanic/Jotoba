use crate::{
    search::{query::Query, SearchMode},
    utils,
};

use super::{jp_parsing::WordItem, result::Word};

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
    }

    if let Some(jlpt) = reading.jlpt_lvl {
        score += jlpt as usize;
    }

    if let Some(morpheme) = morpheme {
        let lexeme = morpheme.get_lexeme();
        if reading.reading == lexeme || kana_reading.reading == lexeme {
            score += 15;
        }
    }

    score
}

pub(super) fn kanji_reading_search(word: &Word, search_order: &SearchOrder) -> usize {
    // TODO implement
    0
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SearchOrder<'a, 'parser> {
    query: &'a Query,
    morpheme: &'a Option<WordItem<'parser, 'a>>,
}

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
