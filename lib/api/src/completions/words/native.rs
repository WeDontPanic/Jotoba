use autocompletion::suggest::{
    extension::{kanji_align::KanjiAlignExtension, similar_terms::SimilarTermsExtension},
    query::SuggestionQuery,
    task::SuggestionTask,
};
use romaji::RomajiExt;

use super::super::*;

/// Get suggestions for foreign search input
pub fn suggestions(query: &Query, radicals: &[char]) -> Option<Vec<WordPair>> {
    let jp_engine = storage::JP_WORD_INDEX.get()?;
    let query_str = query.query.as_str();

    let mut suggestion_task = SuggestionTask::new(30);

    let mut main_sugg_query = SuggestionQuery::new(jp_engine, query_str);

    // Kanji reading align (くにうた ー＞ 国歌)
    let mut k_r_align = KanjiAlignExtension::new(jp_engine);
    k_r_align.options.weights.freq_weight = 10.0;
    k_r_align.options.threshold = 5;
    main_sugg_query.add_extension(k_r_align);

    // Similar terms
    let mut ste = SimilarTermsExtension::new(jp_engine, 7);
    ste.options.threshold = 10;
    main_sugg_query.add_extension(ste);

    suggestion_task.add_query(main_sugg_query);

    // Add katakana results
    if query_str.has_kana() {
        let kanaquery = query_str.to_katakana();
        if kanaquery != query_str {
            let mut kana_query = SuggestionQuery::new(jp_engine, kanaquery);
            kana_query.weights.total_weight = 0.8;
            suggestion_task.add_query(kana_query);
        }
    }

    // radical filter
    let word_res = resources::get().words();
    suggestion_task.set_filter(move |item| {
        if radicals.is_empty() {
            return true;
        }

        let word = word_res.by_sequence(item.word_id()).unwrap();
        word_rad_filter(query_str, word, radicals)
    });

    Some(convert_results(suggestion_task.search()))
}

fn word_rad_filter(query: &str, word: &types::jotoba::words::Word, radicals: &[char]) -> bool {
    let kanji = match word.reading.kanji.as_ref() {
        Some(k) => &k.reading,
        None => return false,
    };

    let retrieve = resources::get().kanji();

    let query_kanji = query.chars().filter(|i| i.is_kanji()).collect::<Vec<_>>();

    kanji
        .chars()
        // Don't apply on existing kanji
        .filter(|i| !query_kanji.contains(&i))
        .filter_map(|k| k.is_kanji().then(|| retrieve.by_literal(k)).flatten())
        .any(|k| {
            if let Some(k_parts) = &k.parts {
                return is_subset(radicals, &k_parts);
            }
            false
        })
}

/// Returns `true` if `subs` is a subset of `full`
pub fn is_subset<T: PartialEq>(subs: &[T], full: &[T]) -> bool {
    if subs.is_empty() || full.is_empty() || subs.len() > full.len() {
        return false;
    }
    for i in subs {
        if !full.contains(i) {
            return false;
        }
    }
    true
}
