use crate::SearchMode;
use japanese::JapaneseExt;
use levenshtein::levenshtein;
use once_cell::sync::Lazy;
use regex::Regex;
use resources::{
    models::{kanji, words::Word},
    parse::jmdict::languages::Language,
};

/// A Regex matching parentheses and its contents
pub(crate) static REMOVE_PARENTHESES: Lazy<Regex> =
    Lazy::new(|| regex::Regex::new("\\(.*\\)").unwrap());

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
        score += 50;

        // Show kana only readings on top if they match with query
        if word.reading.kanji.is_none() {
            score += 10;
        }
    } else if reading.reading.starts_with(query_str) {
        score += 4;
    }

    if let Some(original_query) = original_query {
        if (original_query == reading.reading || original_query == kana)
            && query_str != reading.reading
        {
            score += 20;
        }
    }

    if word.jlpt_lvl.is_some() {
        score += 10;
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
        score += 20;
    }

    score
}

pub fn foreign_search_order(
    word: &Word,
    relevance: f32,
    query_str: &str,
    language: Language,
    user_lang: Language,
) -> usize {
    let mut score: usize = (relevance * 20f32) as usize;

    if word.is_common() {
        score += 10;
    }

    if word.jlpt_lvl.is_some() {
        score += (word.jlpt_lvl.unwrap() * 2) as usize;
    }

    /*
    if !word.is_katakana_word() {
        score += 4;
    }
    */

    // Result found within users specified language
    if language == user_lang {
        score += 12;
    }

    let found = match find_reading(word, query_str) {
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
    kanji_reading: &resources::models::kanji::Reading,
    relevance: f32,
) -> usize {
    let mut score: usize = (relevance * 25f32) as usize;

    // This function should only be called for kanji reading search queries!
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

fn find_reading(word: &Word, query: &str) -> Option<FindResult> {
    for mode in SearchMode::ordered_iter() {
        for ign_case in &[false, true] {
            let res = find_in_senses(&word.senses, query, *mode, *ign_case);
            if res.is_some() {
                return res;
            }
        }
    }

    None
}

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
