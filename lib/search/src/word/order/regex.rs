use crate::query::regex::RegexSQuery;
use types::jotoba::words::Word;
use utils::real_string_len;

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
