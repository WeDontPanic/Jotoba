use autocompletion::suggest::{
    extension::{
        kanji_align::KanjiAlignExtension, ngram::NGramExtension,
        similar_terms::SimilarTermsExtension,
    },
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

    let lang = query.settings.language();

    // Default search query
    task.add_query(new_suggestion_query(&query_lower, lang)?);

    // Add results for english
    if query.settings.show_english() {
        let mut en_sugg_query = new_suggestion_query(&query_lower, Language::English)?;
        en_sugg_query.weights.total_weight = 0.75;
        en_sugg_query.weights.freq_weight = 0.15;
        task.add_query(en_sugg_query);
    }

    // Romaji result
    //if let Some(hira_query) = try_romaji(query_str.trim()) {
    let hira_query = try_romaji(query_str.trim()).unwrap_or_else(|| query_str.to_hiragana());
    //let hira_query = query_str.to_hiragana();
    println!("hira query: {hira_query}");
    let jp_engine = indexes::get_suggestions().jp_words();
    let mut rom_query = SuggestionQuery::new(jp_engine, hira_query);
    rom_query.weights.total_weight = 0.6;
    /*
    query.weights.freq_weight = 0.1;
    query.weights.str_weight = 1.9;
    */

    let mut k_r_align = KanjiAlignExtension::new(jp_engine);
    k_r_align.options.weights.freq_weight = 1.0;
    k_r_align.options.threshold = 5;
    rom_query.add_extension(k_r_align);

    let mut similar_terms = SimilarTermsExtension::new(jp_engine, 14);
    similar_terms.options.threshold = 10;
    similar_terms.options.weights.total_weight = 0.75;
    similar_terms.options.weights.freq_weight = 0.2;
    similar_terms.options.weights.str_weight = 1.8;
    similar_terms.options.min_query_len = 4;
    rom_query.add_extension(similar_terms);

    let mut ng_ext = NGramExtension::with_sim_threshold(jp_engine, 0.35);
    //ng_ext.options.threshold = 10;
    ng_ext.options.weights.total_weight = 0.25;
    ng_ext.options.weights.freq_weight = 0.04;
    ng_ext.query_weigth = 0.85;
    ng_ext.term_limit = 10_000;
    //ng_ext.options.weights.total_weight = 0.1;
    ng_ext.options.min_query_len = 5;
    ng_ext.cust_query = Some(query_str);
    rom_query.add_extension(ng_ext);

    task.set_rel_mod(|i, rel| {
        let out = i.to_output();
        let kana = &out.primary;
        if japanese::romaji_prefix(query_str.trim(), &kana) {
            return rel + 1000;
        }
        rel
    });

    task.add_query(rom_query);
    //}

    Some(convert_results(task.search()))
}

fn new_suggestion_query(query: &str, lang: Language) -> Option<SuggestionQuery> {
    let engine = indexes::get_suggestions().foreign_words(lang)?;

    let mut suggestion_query = SuggestionQuery::new(engine, &query);
    suggestion_query.weights.str_weight = 1.5;
    suggestion_query.weights.freq_weight = 0.5;

    let mut ng_ex = NGramExtension::with_sim_threshold(engine, 0.5);
    ng_ex.options.weights.total_weight = 0.7;
    ng_ex.options.weights.freq_weight = 0.05;
    ng_ex.query_weigth = 0.7;
    ng_ex.options.min_query_len = 4;
    suggestion_query.add_extension(ng_ex);

    Some(suggestion_query)
}

/// Returns Some(String) if `query_str` could be (part of) romaji search input and None if not
pub(crate) fn try_romaji(query_str: &str) -> Option<String> {
    let mut query_str = query_str.replace("-", "ー");
    if query_str.ends_with("m") {
        query_str.pop();
    }
    let query_str = &query_str;

    let str_len = real_string_len(query_str);
    if str_len < 3 || query_str.contains(' ') {
        return None;
    }

    if let Some(v) = is_romaji_repl(query_str) {
        return Some(v.to_hiragana());
    }

    if str_len < 3 {
        return None;
    }

    // 'n' is the only hiragana with with=1 in romaji so allow them
    // to be treated properly too
    let min_len = 3 - query_str.chars().filter(|i| *i == 'n').count();

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
