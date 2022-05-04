use autocompletion::suggest::{
    extension::{longest_prefix::LongestPrefixExtension, similar_terms::SimilarTermsExtension},
    query::SuggestionQuery,
    task::SuggestionTask,
};
use japanese::guessing::is_romaji_repl;
use types::jotoba::languages::Language;
use utils::real_string_len;

use super::super::*;

/// Returns suggestions based on non japanese input
pub fn suggestions(query: &Query, query_str: &str) -> Option<Vec<WordPair>> {
    let query_lower = autocompletion::index::basic::basic_format(query_str.trim());
    let mut task = SuggestionTask::new(30);

    //println!("raw: {:?}", query_lower.trim());

    // Raw

    // Default search query
    task.add_query(new_suggestion_query(
        &query_lower,
        query.settings.user_lang,
    )?);

    // Add results for english
    if query.settings.show_english() {
        let mut en_sugg_query = new_suggestion_query(&query_lower, Language::English)?;
        en_sugg_query.weights.total_weight = 0.3;
        en_sugg_query.weights.freq_weight = 0.2;
        task.add_query(en_sugg_query);
    }

    // Romaji result
    if let Some(hira_query) = try_romaji(query_str.trim()) {
        if let Some(jp_engine) = storage::JP_WORD_INDEX.get() {
            let mut query = SuggestionQuery::new(jp_engine, hira_query);
            query.weights.total_weight = 0.5;

            let mut similar_terms = SimilarTermsExtension::new(jp_engine, 5);
            similar_terms.options.weights.total_weight = 0.4;
            similar_terms.options.threshold = 0;
            query.add_extension(similar_terms);

            task.add_query(query);
        }
    }

    Some(convert_results(task.search()))
}

fn new_suggestion_query(query: &str, lang: Language) -> Option<SuggestionQuery> {
    let engine = storage::WORD_INDEX.get()?.get(&lang)?;

    let mut suggestion_query = SuggestionQuery::new(engine, &query);
    suggestion_query.weights.str_weight = 1.5;
    suggestion_query.weights.freq_weight = 0.5;

    let mut ste = SimilarTermsExtension::new(engine, 5);
    ste.options.threshold = 10;
    ste.options.weights.freq_weight = 1.0;
    ste.options.weights.str_weight = 1.0;
    ste.options.weights.total_weight = 0.05;
    //suggestion_query.add_extension(ste);

    let mut lpe = LongestPrefixExtension::new(engine, 3, 10);
    lpe.options.threshold = 10;
    lpe.options.weights.freq_weight = 1.0;
    lpe.options.weights.total_weight = 0.3;
    suggestion_query.add_extension(lpe);

    Some(suggestion_query)
}

/// Returns Some(String) if `query_str` could be (part of) romaji search input and None if not
pub(crate) fn try_romaji(query_str: &str) -> Option<String> {
    let str_len = real_string_len(query_str);
    if str_len < 3 || query_str.contains(' ') {
        return None;
    }

    if let Some(v) = is_romaji_repl(query_str) {
        return Some(v.to_hiragana());
    }

    if str_len < 4 {
        return None;
    }

    // 'n' is the only hiragana with with=1 in romaji so allow them
    // to be treated properly too
    let min_len = 4 - query_str.chars().filter(|i| *i == 'n').count();

    // Strip one to avoid switching between romaji/normal results
    if str_len > min_len {
        let prefix = strip_str_end(query_str, 1);
        if let Some(v) = is_romaji_repl(prefix) {
            return Some(v.to_hiragana());
        }
    }

    // shi ending needs more stripping but also more existing romaji to not
    // heavily overlap with other results
    if str_len >= min_len + 2 && end_three_char_kana(query_str) {
        let prefix = strip_str_end(query_str, 2);
        if let Some(v) = is_romaji_repl(prefix) {
            return Some(v.to_hiragana());
        }
    }

    None
}

/// Returns a substring of `inp` with `len` amount of tailing characters being removed.
/// This works for non UTF-8 as well. If len > |inp| "" gets returned
#[inline]
pub fn strip_str_end(inp: &str, len: usize) -> &str {
    match inp.char_indices().rev().nth(len - 1).map(|i| i.0) {
        Some(end) => &inp[..end],
        None => "",
    }
}

/// Returns `true` if `s` ends with 2 of 3 3-char kana romaji
#[inline]
fn end_three_char_kana(s: &str) -> bool {
    [
        "sh", "ch", "ts", "hy", "ky", "ny", "my", "gy", "ry", "by", "py",
    ]
    .iter()
    .any(|i| s.ends_with(i))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strip_end() {
        let inp = "これはかっこいいテキスト";
        assert_eq!(strip_str_end(inp, 1), "これはかっこいいテキス");
        assert_eq!(strip_str_end(inp, 2), "これはかっこいいテキ");
        assert_eq!(strip_str_end(inp, 3), "これはかっこいいテ");
    }
}
