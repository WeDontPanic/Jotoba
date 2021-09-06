use std::collections::HashMap;

use super::super::search_order::SearchOrder;
use crate::{engine::result::ResultItem, query::Query, SearchMode};
use japanese::JapaneseExt;
use levenshtein::levenshtein;
//use models::search_mode::SearchMode;
use once_cell::sync::Lazy;
use regex::Regex;
use resources::models::{kanji, words::Word};

pub(super) fn new_foreign_order(
    sort_map: &HashMap<usize, ResultItem>,
    search_order: &SearchOrder,
    e: &mut Vec<Word>,
) {
    e.sort_by(|a, b| {
        let a_item = sort_map.get(&(a.sequence as usize)).unwrap();
        let b_item = sort_map.get(&(b.sequence as usize)).unwrap();

        let a_score = foreign_search_order(a, search_order, a_item);
        let b_score = foreign_search_order(b, search_order, b_item);

        a_score.cmp(&b_score).reverse()
    })
}

pub(super) fn foreign_search_order(
    word: &Word,
    search_order: &SearchOrder,
    result_item: &ResultItem,
) -> usize {
    let mut score: usize = (result_item.relevance * 25f32) as usize;

    if word.is_common() {
        score += 10;
    }

    if let Some(jlpt) = word.jlpt_lvl {
        score += (jlpt as usize) * 1;
    }

    if !word.is_katakana_word() {
        score += 8;
    }

    // Result found within users specified language
    if result_item.language == search_order.query.settings.user_lang {
        score += 12;
    }

    let found = match find_reading(word, &search_order.query) {
        Some(v) => v,
        None => {
            return score;
        }
    };

    let divisor = match (found.mode, found.case_ignored) {
        (SearchMode::Exact, false) => 10,
        (SearchMode::Exact, true) => 20,
        (_, false) => 50,
        (_, true) => 80,
    };

    score += (calc_likeliness(word, &found) / divisor) as usize;

    if found.in_parentheses {
        score = score - score.clamp(0, 10);
    } else {
        score += 30;
    }

    score
}

pub(super) fn new_japanese_order(
    sort_map: &HashMap<usize, ResultItem>,
    search_order: &SearchOrder,
    e: &mut Vec<Word>,
) {
    e.sort_by(|a, b| {
        let a_item = sort_map.get(&(a.sequence as usize)).unwrap();
        let b_item = sort_map.get(&(b.sequence as usize)).unwrap();

        let a_score = japanese_search_order(a, search_order, a_item);
        let b_score = japanese_search_order(b, search_order, b_item);

        a_score.cmp(&b_score).reverse()
    })
}

/// Search order for words searched by japanese meaning/kanji/reading
pub(super) fn japanese_search_order(
    word: &Word,
    search_order: &SearchOrder,
    result_item: &ResultItem,
) -> usize {
    let mut score: usize = (result_item.relevance * 10f32) as usize;

    let reading = word.get_reading();

    // Original query
    let query = search_order.query;
    // The original query text
    let query_str = &query.query;

    if reading.reading == *query_str || word.reading.kana.reading == *query_str {
        score += 50;

        // Show kana only readings on top if they match with query
        if word.reading.kanji.is_none() {
            score += 10;
        }
    } else if reading.reading.starts_with(query_str) {
        score += 4;
    }

    if let Some(jlpt) = word.jlpt_lvl {
        score += (jlpt as usize) * 2;
    }

    // Is common
    if word.is_common() {
        score += 20;
    }

    // If alternative reading matches query exactly
    if word
        .reading
        .alternative
        .iter()
        .any(|i| i.reading == *query_str)
    {
        score += 45;
    }

    score
}

pub(super) fn new_kanji_reading_search_order(
    sort_map: &HashMap<usize, ResultItem>,
    search_order: &SearchOrder,
    e: &mut Vec<Word>,
) {
    e.sort_by(|a, b| {
        let a_item = sort_map.get(&(a.sequence as usize)).unwrap();
        let b_item = sort_map.get(&(b.sequence as usize)).unwrap();

        let a_score = kanji_reading_search(a, search_order, a_item);
        let b_score = kanji_reading_search(b, search_order, b_item);

        a_score.cmp(&b_score).reverse()
    })
}
pub(super) fn kanji_reading_search(
    word: &Word,
    search_order: &SearchOrder,
    result_item: &ResultItem,
) -> usize {
    let mut score: usize = (result_item.relevance * 25f32) as usize;

    // This function should only be called for kanji reading search queries!
    debug_assert!(search_order.query.form.as_kanji_reading().is_some());
    let kanji_reading = search_order.query.form.as_kanji_reading().unwrap();
    let formatted_reading = kanji::format_reading(&kanji_reading.reading);
    let kana_reading = &word.reading.kana.reading;

    if formatted_reading.is_hiragana() {
        // Kun reading
        if *kana_reading == formatted_reading
            // Don't show direct readings if the kanji reading is a suffix/prefix
            && !kanji_reading.reading.starts_with('-')
            && !kanji_reading.reading.ends_with('-')
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

    if let Some(jlpt) = word.jlpt_lvl {
        score += jlpt as usize;
    }

    score
}

fn calc_likeliness(this: &Word, fres: &FindResult) -> u8 {
    let gloss_len: usize = this
        .senses
        .iter()
        // Ignore other languages
        .filter_map(|i| (fres.language == i.language).then(|| i.glosses.len()))
        .sum();

    100 - ((fres.sense_pos * 100) / gloss_len) as u8
}

pub fn get_query_pos_in_gloss(
    query_str: &String,
    this: &Word,
    s_mode: SearchMode,
    ign_case: bool,
) -> Option<usize> {
    for lang_senes in this.get_senses() {
        let res = find_in_senses(&lang_senes, query_str, s_mode, ign_case);
        if let Some(res) = res {
            return Some(res.pos);
        }
    }

    None
}

#[derive(Debug, Clone)]
struct FindResult {
    mode: SearchMode,
    case_ignored: bool,
    language: resources::parse::jmdict::languages::Language,
    pos: usize,
    gloss: String,
    in_parentheses: bool,
    sense: resources::models::words::Sense,
    sense_pos: usize,
}

fn find_reading(word: &Word, query: &Query) -> Option<FindResult> {
    let query_str = &query.query;

    for mode in SearchMode::ordered_iter() {
        for ign_case in &[false, true] {
            let res = find_in_senses(&word.senses, query_str, *mode, *ign_case);
            if res.is_some() {
                return res;
            }
        }
    }

    None
}

/// A Regex matching parentheses and its contents
pub(crate) static REMOVE_PARENTHESES: Lazy<Regex> =
    Lazy::new(|| regex::Regex::new("\\(.*\\)").unwrap());

fn find_in_senses(
    senses: &[resources::models::words::Sense],
    query_str: &str,
    mode: SearchMode,
    ign_case: bool,
) -> Option<FindResult> {
    for (pos, sense) in senses.iter().enumerate() {
        let mut found = try_find_in_sense(&sense, query_str, mode, ign_case, true);
        let in_parentheses = found.is_none();

        if found.is_none() {
            found = try_find_in_sense(&sense, query_str, mode, ign_case, false);
            if found.is_none() {
                continue;
            }
        }

        let (sense_pos, gloss) = found.unwrap();

        return Some(FindResult {
            mode,
            pos,
            language: sense.language,
            case_ignored: ign_case,
            gloss,
            in_parentheses,
            sense: sense.clone(),
            sense_pos,
        });
    }

    None
}

fn try_find_in_sense(
    sense: &resources::models::words::Sense,
    query_str: &str,
    mode: SearchMode,
    ign_case: bool,
    ign_parentheses: bool,
) -> Option<(usize, String)> {
    sense.glosses.iter().enumerate().find_map(|(pos, g)| {
        let gloss = if ign_parentheses {
            let gloss = REMOVE_PARENTHESES.replace(&g.gloss, "");
            gloss.trim().to_string()
        } else {
            g.gloss.to_owned()
        };
        mode.str_eq(&gloss.as_str(), &query_str, ign_case)
            .then(|| (pos, gloss.to_owned()))
    })
}
