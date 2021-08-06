use std::collections::HashMap;

use japanese::JapaneseExt;
use levenshtein::levenshtein;
use once_cell::sync::Lazy;
use parse::jmdict::languages::Language;
use regex::Regex;

use crate::query::Query;

use super::engine::result::ResultItem;
use super::result::Sense;
use super::{super::search_order::SearchOrder, result::Word};
use models::kanji;
use models::search_mode::SearchMode;

pub(super) fn new_foreign_order(
    sort_map: &HashMap<usize, ResultItem>,
    search_order: &SearchOrder,
    e: &mut Vec<Word>,
) {
    e.sort_by(|a, b| {
        let a_item = sort_map.get(&(a.sequence as usize)).unwrap();
        let b_item = sort_map.get(&(b.sequence as usize)).unwrap();

        foreign_search_order(a, search_order, a_item)
            .cmp(&foreign_search_order(b, search_order, b_item))
            .reverse()
    })
}

pub(super) fn foreign_search_order(
    word: &Word,
    search_order: &SearchOrder,
    result_item: &ResultItem,
) -> usize {
    let mut score: usize = (result_item.relevance * 25f32) as usize;
    let reading = word.get_reading();

    if word.is_common() {
        score += 10;
    }

    if let Some(jlpt) = reading.jlpt_lvl {
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

    /*
    let divisor = match (found.mode, found.case_ignored) {
        (SearchMode::Exact, false) => 10,
        (SearchMode::Exact, true) => 30,
        (_, false) => 50,
        (_, true) => 80,
    };

    score += (calc_likeliness(word, &found) / divisor) as usize;
    */

    if found.in_parentheses {
        score = score - score.clamp(0, 10);
    } else {
        score += 30;
    }

    score
}

pub(super) fn new_native_order(
    sort_map: &HashMap<usize, ResultItem>,
    search_order: &SearchOrder,
    e: &mut Vec<Word>,
) {
    e.sort_by(|a, b| {
        let a_item = sort_map.get(&(a.sequence as usize)).unwrap();
        let b_item = sort_map.get(&(b.sequence as usize)).unwrap();

        native_search_order(a, search_order, a_item)
            .cmp(&foreign_search_order(b, search_order, b_item))
            .reverse()
    })
}

/// Search order for words searched by japanese meaning/kanji/reading
pub(super) fn native_search_order(
    word: &Word,
    search_order: &SearchOrder,
    result_item: &ResultItem,
) -> usize {
    #[cfg(feature = "tokenizer")]
    let morpheme = search_order.morpheme;

    let reading = word.get_reading();
    let kana_reading = word.reading.kana.as_ref().unwrap();
    // Original query
    let query = search_order.query;
    // The original query text
    let query_str = &query.query;

    let mut score: usize = (result_item.relevance * 25f32) as usize;
    //let mut score = 0;

    if reading.reading == *query_str || kana_reading.reading == *query_str {
        score += 35;

        // Show kana only readings on top if they match with query
        if word.reading.kanji.is_none() {
            score += 20;
        }
    } else if reading.reading.starts_with(query_str) {
        score += 2;
    }

    if let Some(jlpt) = reading.jlpt_lvl {
        score += (jlpt as usize) * 2;
    }

    // If alternative reading matches query exactly
    if word
        .reading
        .alternative
        .iter()
        .any(|i| i.reading == *query_str)
    {
        score += 9;
    }

    #[cfg(feature = "tokenizer")]
    if let Some(morpheme) = morpheme {
        let lexeme = morpheme.get_lexeme();
        if reading.reading == lexeme || kana_reading.reading == lexeme {
            score += 15;
        }

        // Show kana only readings on top if they match with lexeme
        if word.reading.kanji.is_none() {
            score += 30;
        }
    }

    // Is common
    if word.is_common() {
        score += 15;
    }

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

    if let Some(jlpt) = word.get_reading().jlpt_lvl {
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
    language: Language,
    pos: usize,
    gloss: String,
    in_parentheses: bool,
    sense: Sense,
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
    senses: &[Sense],
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
    sense: &Sense,
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
