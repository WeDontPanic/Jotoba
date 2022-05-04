use crate::{regex_query::RegexSQuery, SearchMode};
use japanese::JapaneseExt;
use levenshtein::levenshtein;
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
pub fn japanese_search_order(
    word: &Word,
    relevance: f32,
    query_str: &str,
    original_query: Option<&str>,
) -> usize {
    let mut score: usize = (relevance * 10f32) as usize;

    let reading = word.get_reading();
    let kana = &word.reading.kana.reading;

    if reading.reading == *query_str || word.reading.kana.reading == *query_str {
        score += 80;

        // Show kana only readings on top if they match with query
        if word.reading.kanji.is_none() {
            score += 10;
        }
    } else if reading.reading.starts_with(query_str) {
        score += 4;
    }

    if let Some(original_query) = original_query {
        if original_query == reading.reading || original_query == kana
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

    if word.get_reading().reading.starts_with(query_str)
        || (query_str.is_kana() && word.reading.kana.reading.starts_with(query_str))
    {
        score += 20;
    }

    // If alternative reading matches query exactly
    if word
        .reading
        .alternative
        .iter()
        .any(|i| i.reading == *query_str)
    {
        score += 60;
    }

    score
}

pub fn foreign_search_order(
    word: &Word,
    relevance: f32,
    query_str: &str,
    query_lang: Language,
    user_lang: Language,
) -> usize {
    let mut score = 0f64; //relevance as f64 * 10.0;

    let found = match find_reading(word, query_str, user_lang, query_lang) {
        Some(v) => v,
        None => {
            return score as usize;
        }
    };

    // Each gloss considered in a frequency analysis has been normalized to 1. Thus we require it
    // to be 1 or more. Otherwise run the fall-back scoring method
    if found.gloss_full.occurrence >= 1 {
        // found.sense.language == language &&
        score += found.gloss_full.occurrence as f64;
    } else {
        return foreign_search_fall_back(word, relevance, query_str, query_lang, user_lang);
    }

    let mut multiplicator = match (found.mode, found.case_ignored) {
        (SearchMode::Exact, _) => 100,
        (_, false) => 10,
        (_, true) => 8,
    };

    // Result found within users specified language
    if query_lang != user_lang {
        //score += 1000.0;
        multiplicator -= 8;
    } else {
        multiplicator *= 2;
    }

    score *= multiplicator as f64;

    if word.is_common() {
        //score += 10.0;
    }

    if !found.in_parentheses {
        score -= 10f64.min(score);
        //score = score.saturating_sub(10.0);
    } else {
        score += 30.0;
    }

    score as usize
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
        (SearchMode::Exact, true) => 10,
        (_, false) => 50,
        (_, true) => 80,
    };

    score += (calc_likeliness(word, &found) / divisor) as usize;

    if found.in_parentheses {
        score = score.saturating_sub(10);
    } else {
        score += 30;
    }

    score
}

pub(super) fn kanji_reading_search(
    word: &Word,
    kanji_reading: &types::jotoba::kanji::ReadingSearch,
    relevance: f32,
) -> usize {
    let mut score: usize = (relevance * 25f32) as usize;

    // This function should only be called for kanji reading search queries!
    let formatted_reading = types::jotoba::kanji::format_reading(&kanji_reading.reading);
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
    gloss: String,
    in_parentheses: bool,
    sense: types::jotoba::words::sense::Sense,
    sense_pos: usize,
    gloss_full: Gloss,
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

        let (sense_pos, gloss_str, gloss) = found.unwrap();
        let curr_occurrence = gloss.occurrence;

        let this_res = Some(FindResult {
            mode,
            pos,
            language: sense.language,
            case_ignored: ign_case,
            gloss: gloss_str,
            in_parentheses,
            sense: sense.clone(),
            sense_pos,
            gloss_full: gloss,
        });

        if let Some(ref curr_res) = res {
            if curr_res.gloss_full.occurrence < curr_occurrence {
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
