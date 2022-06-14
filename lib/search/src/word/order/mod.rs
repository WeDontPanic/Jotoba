pub mod foreign;

use crate::{engine::search_task::sort_item::SortItem, query::regex::RegexSQuery, SearchMode};
use japanese::JapaneseExt;
use once_cell::sync::Lazy;
use regex::Regex;
use types::jotoba::{
    languages::Language,
    words::{sense::Gloss, Word},
};
use utils::real_string_len;

/// A Regex matching parentheses and its contents
pub(crate) static REMOVE_PARENTHESES: Lazy<Regex> =
    Lazy::new(|| regex::Regex::new("\\(.*\\)").unwrap());

/// Order for regex-search results
pub fn regex_order(word: &Word, found_in: &str, _query: &RegexSQuery) -> usize {
    let mut score: usize = 100;

    if !word
        .reading
        .alternative
        .iter()
        .any(|i| i.reading == found_in)
    {
        score += 20;
    }

    if word.is_common() {
        score += 30;
    }

    if let Some(jlpt) = word.get_jlpt_lvl() {
        score += 10 + (jlpt * 2) as usize;
    }

    // Show shorter words more on top
    score = score.saturating_sub(real_string_len(&word.get_reading().reading) * 3);

    score
}

/// Search order for words searched by japanese meaning/kanji/reading
pub fn japanese_search_order(item: SortItem<&'static Word>, original_query: Option<&str>) -> usize {
    let mut score: usize = (item.vec_simiarity() * 10f32) as usize;

    let word = item.item();
    let query_str = japanese::to_halfwidth(item.query());

    let reading = japanese::to_halfwidth(&word.get_reading().reading);
    let kana = japanese::to_halfwidth(&word.reading.kana.reading);

    if reading == *query_str || kana == *query_str {
        score += 80;

        // Show kana only readings on top if they match with query
        if word.reading.kanji.is_none() {
            score += 10;
        }
    } else if reading.starts_with(&query_str) {
        score += 4;
    }

    if let Some(original_query) = original_query {
        if original_query == reading || original_query == kana
        //&& query_str != reading.reading
        {
            score += 500;
        }
    }

    if word.jlpt_lvl.is_some() {
        score += 10;
    }

    // Is common
    if word.is_common() {
        score += 20;
    }

    if reading.starts_with(&query_str) || (query_str.is_kana() && reading.starts_with(&query_str)) {
        score += 20;
    }

    // If alternative reading matches query exactly
    if word
        .reading
        .alternative
        .iter()
        .map(|i| japanese::to_halfwidth(&i.reading))
        .any(|i| i == *query_str)
    {
        score += 60;
    }

    score
}

pub fn foreign_search_fall_back(
    word: &Word,
    relevance: f32,
    query_str: &str,
    query_lang: Language,
    user_lang: Language,
) -> usize {
    let mut score: usize = (relevance * 20f32) as usize;

    if word.is_common() {
        score += 10;
    }

    if word.jlpt_lvl.is_some() {
        score += (word.jlpt_lvl.unwrap() * 2) as usize;
    }

    // Result found within users specified language
    if query_lang == user_lang {
        score += 12;
    }

    let found = match find_reading(word, query_str, user_lang, query_lang) {
        Some(v) => v,
        None => {
            return score;
        }
    };

    let divisor = match (found.mode, found.case_ignored) {
        (SearchMode::Exact, false) => 10,
        (SearchMode::Exact, true) => 7,
        (_, false) => 4,
        (_, true) => 3,
    };

    score += (calc_likeliness(word, &found) / divisor) as usize;

    if found.in_parentheses {
        score = score.saturating_sub(10);
    } else {
        score += 30;
    }

    score
}

pub(super) fn kanji_reading_search(item: SortItem<&'static Word>) -> usize {
    let word = item.item();
    let mut score: usize = 0;

    if word.is_common() {
        score += 100;
    }

    if let Some(jlpt) = word.jlpt_lvl {
        score += jlpt as usize * 10;
    }

    if score == 0 {
        // Show shorter words on top if they aren't important
        let reading_len = word.reading.get_reading().reading.chars().count();
        score = 100usize.saturating_sub(reading_len * 2);
    } else {
        score += 100;
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
        let res = find_in_senses(
            &lang_senes,
            query_str,
            s_mode,
            ign_case,
            Language::English,
            Language::English,
        );
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
    language: types::jotoba::languages::Language,
    pos: usize,
    in_parentheses: bool,
    sense_pos: usize,
}

fn find_reading(
    word: &Word,
    query: &str,
    user_lang: Language,
    expected_lang: Language,
) -> Option<FindResult> {
    for mode in SearchMode::ordered_iter() {
        for ign_case in &[false, true] {
            let res = find_in_senses(
                &word.senses,
                query,
                *mode,
                *ign_case,
                user_lang,
                expected_lang,
            );

            if res.is_some() {
                return res;
            }
        }
    }

    None
}

fn find_in_senses(
    senses: &[types::jotoba::words::sense::Sense],
    query_str: &str,
    mode: SearchMode,
    ign_case: bool,
    user_lang: Language,
    expected_lang: Language,
) -> Option<FindResult> {
    let mut res: Option<FindResult> = None;
    for (pos, sense) in senses.iter().enumerate() {
        if sense.language != expected_lang && sense.language != user_lang {
            continue;
        }
        let mut found = try_find_in_sense(&sense, query_str, mode, ign_case, true);
        let in_parentheses = found.is_none();

        if found.is_none() {
            found = try_find_in_sense(&sense, query_str, mode, ign_case, false);
            if found.is_none() {
                continue;
            }
        }

        let (sense_pos, _, _) = found.unwrap();
        //let curr_occurrence = gloss.occurrence;
        let curr_occurrence = 0;

        let this_res = Some(FindResult {
            mode,
            pos,
            language: sense.language,
            case_ignored: ign_case,
            in_parentheses,
            sense_pos,
        });

        if let Some(ref _curr_res) = res {
            if 1/*curr_res.gloss_full.occurrence*/ < curr_occurrence {
                res = this_res;
            }
        } else {
            res = this_res;
        }
    }

    res
}

fn try_find_in_sense(
    sense: &types::jotoba::words::sense::Sense,
    query_str: &str,
    mode: SearchMode,
    ign_case: bool,
    ign_parentheses: bool,
) -> Option<(usize, String, Gloss)> {
    sense.glosses.iter().enumerate().find_map(|(pos, g)| {
        let gloss = if ign_parentheses {
            let gloss = REMOVE_PARENTHESES.replace(&g.gloss, "");
            gloss.trim().to_string()
        } else {
            g.gloss.to_owned()
        };
        mode.str_eq(&gloss.as_str(), &query_str, ign_case)
            .then(|| (pos, gloss.to_owned(), g.clone()))
    })
}
