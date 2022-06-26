use super::{super::*, kana_end_ext::KanaEndExtension};
use autocompletion::{
    index::{str_item::StringItem, IndexItem},
    suggest::{
        extension::{
            kanji_align::KanjiAlignExtension, ngram::NGramExtension,
            similar_terms::SimilarTermsExtension,
        },
        query::SuggestionQuery,
        task::SuggestionTask,
    },
};
use romaji::RomajiExt;

const MAX_SENTENCE_LEN: usize = 15;

/// Get suggestions for foreign search input
pub fn suggestions(query: &Query, romaji_query: &str, radicals: &[char]) -> Option<Vec<WordPair>> {
    let jp_engine = indexes::get_suggestions().jp_words();
    let query_str = query.query.as_str();

    let mut suggestion_task = SuggestionTask::new(30);

    let mut main_sugg_query = SuggestionQuery::new(jp_engine, query_str);
    main_sugg_query.weights.str_weight = 1.2;

    // Kanji reading align (くにうた ー＞ 国歌)
    let mut k_r_align = KanjiAlignExtension::new(jp_engine);
    k_r_align.options.weights.freq_weight = 1.0;
    k_r_align.options.threshold = 5;
    main_sugg_query.add_extension(k_r_align);

    // Find 天気予報 even if 天気よほう was written
    let mut kana_end_ext = KanaEndExtension::new(jp_engine, 10);
    kana_end_ext.options.weights.total_weight = 0.45;
    kana_end_ext.options.weights.freq_weight = 0.4;
    main_sugg_query.add_extension(kana_end_ext);

    // Similar terms based on pronounciation
    let mut ste = SimilarTermsExtension::new(jp_engine, 16);
    ste.options.threshold = 10;
    ste.options.weights.total_weight = 0.6;
    ste.options.weights.freq_weight = 0.5;
    //ste.options.weights.str_weight = 1.4;
    main_sugg_query.add_extension(ste);

    // Fix typos
    let mut ng_ex = NGramExtension::with_sim_threshold(jp_engine, 0.5);
    ng_ex.options.weights.freq_weight = 0.05;
    ng_ex.query_weigth = 0.7;
    ng_ex.cust_query = Some(&romaji_query);

    main_sugg_query.add_extension(ng_ex);

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

    let (norm_form, sentence) = normalize_inflections(query_str);
    if let Some(normalized) = norm_form {
        let mut norm_query = SuggestionQuery::new(jp_engine, normalized);
        norm_query.threshold = 2;
        norm_query.weights.total_weight = 0.01;
        norm_query.weights.freq_weight = 0.0;
        suggestion_task.add_query(norm_query);
    }

    let sentence_len = sentence.len();
    let items: Vec<_> = sentence
        .into_iter()
        .map(|w| StringItem::new(w, 0.0))
        .collect();
    let items: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(pos, i)| {
            let mut engine_item = i.into_engine_item();
            engine_item.set_relevance((sentence_len - pos) as u16);
            engine_item
        })
        .collect();
    if sentence_len > 0 && sentence_len <= MAX_SENTENCE_LEN {
        suggestion_task.add_custom_entries(items);
    }

    // radical filter
    let word_res = resources::get().words();
    suggestion_task.set_filter(move |item| {
        if radicals.is_empty() {
            return true;
        }

        let word = match word_res.by_sequence(item.word_id()) {
            Some(word) => word,
            None => return true,
        };
        word_rad_filter(query_str, word, radicals)
    });

    Some(convert_results(suggestion_task.search()))
}

fn normalize_inflections(query_str: &str) -> (Option<String>, Vec<String>) {
    let parse_res = sentence_reader::Parser::new(query_str).parse();

    if let sentence_reader::output::ParseResult::InflectedWord(word) = parse_res {
        return (Some(word.get_normalized()), vec![]);
    }

    if let sentence_reader::output::ParseResult::Sentence(sentence) = parse_res {
        let items: Vec<_> = sentence
            .iter()
            .filter_map(|i| {
                let wc = i.word_class_raw();
                if wc.is_space() || wc.is_symbol() || wc.is_particle() {
                    return None;
                }
                Some(i.get_normalized())
            })
            .collect();
        return (None, items);
    }

    (None, vec![])
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
            if !k.parts.is_empty() {
                return utils::part_of(radicals, &k.parts);
            }
            false
        })
}
