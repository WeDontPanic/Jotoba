pub mod foreign;

use crate::{query::regex::RegexSQuery, SearchMode};
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

/*
fn make_search_vec(indexer: &TermIndexer, query: &str) -> Option<Vector> {
    let terms: Vec<_> = query
        .split(' ')
        .filter_map(|s_term| Some((s_term, indexer.get_term(s_term)?)))
        .map(|(_, dim)| (dim as u32, 1.0))
        .collect();

    if terms.is_empty() {
        return None;
    }

    Some(Vector::create_new_raw(terms))
}

fn overlapping_vals(src_vec: &Vector, query: &Vector) -> f32 {
    if !src_vec.could_overlap(query) {
        return 0.0;
    }

    let overlapping = src_vec.overlapping(query).map(|i| i.1).collect::<Vec<_>>();
    let sum: f32 = overlapping.iter().sum();
    let div = src_vec.sparse_vec().len().max(query.sparse_vec().len());
    let overlapping_relevance = (overlapping.len() as f32 / div as f32) * 5.0;
    overlapping_relevance + sum * 2.0
}

fn gloss_relevance(query_str: &str, seq_id: u32, sense: &Sense, gloss: &Gloss) -> Option<usize> {
    let index = engine::words::foreign::index::get(sense.language)?;
    let rel_index = engine::words::foreign::index::RELEVANCE_INDEXES
        .get()?
        .get(&sense.language)?;

    let indexer = index.get_indexer().clone();
    let query_vec = make_search_vec(&indexer, query_str)?;

    let sg_id = to_unique_id(sense.id, gloss.id);
    let rel_vec = rel_index.get(seq_id, sg_id)?;
    let val = (overlapping_vals(rel_vec, &query_vec)) as usize;
    Some(val)
}

fn foreign_search_order(
    word_output: &WordOutput,
    relevance: f32,
    query_str: &str,
    query_lang: Language,
    user_lang: Language,
) -> usize {
    let text_score = (relevance as f64 * 10000.0) as usize;

    let word = word_output.word;

    let query_str = query_str.trim().to_lowercase();

    let gloss_relevance = word_output
        .position_iter()
        .filter_map(|(s_id, g_id, _)| {
            let sense = word.sense_by_id(s_id).expect("Failed to get sense");
            let gloss = sense.gloss_by_id(g_id).expect("Failed to get gloss");
            Some((sense, gloss))
        })
        .filter_map(|(sense, gloss)| gloss_relevance(&query_str, word.sequence, sense, gloss))
        .map(|i| i + 1000)
        //   .inspect(|i| println!("{:?}: {}", word.get_reading().reading, i))
        .max()
        .unwrap_or_else(|| {
            foreign_search_fall_back(word, relevance, &query_str, query_lang, user_lang)
        });

    gloss_relevance + text_score
}
*/

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

pub(super) fn kanji_reading_search(
    word: &Word,
    kanji_reading: &types::jotoba::kanji::reading::ReadingSearch,
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
