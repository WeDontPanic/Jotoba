use engine::Engine;
use index_framework::traits::{
    backend::Backend, dictionary::IndexDictionary, postings::IndexPostings,
};
use japanese::ToKanaExt;
use order_struct::order_nh::OrderVal;
use priority_container::PrioContainerMax;
use search::engine::words::native::k_reading;
use types::{
    api::app::completions::{Response, SuggestionType, WordPair},
    jotoba::kanji,
};

/// Gets suggestions for kanji reading search eg: "痛 いた.い"
pub fn suggestions(kanji_reading: kanji::reading::ReadingSearch) -> Option<Response> {
    let kanji_storage = resources::get().kanji();

    let query_reading = kanji_reading
        .reading
        .replace("。", "")
        .replace(".", "")
        .to_hiragana();

    let kanji = kanji_storage.by_literal(kanji_reading.literal)?;

    let mut queue = PrioContainerMax::new(30);

    let iter = kanji
        .kunyomi
        .iter()
        .chain(kanji.onyomi.iter())
        .map(|i| WordPair::with_secondary(i.clone(), kanji.literal.to_string()))
        .map(|wp| {
            let score = score(kanji.literal, &wp.primary, &query_reading);
            OrderVal::new(wp, score)
        });
    queue.extend(iter);

    if queue.is_empty() {
        return None;
    }

    let mut vec: Vec<_> = queue.into_iter().map(|i| i.0.into_inner()).collect();
    vec.reverse();

    Some(Response::with_type(vec, SuggestionType::KanjiReading))
}

fn score(literal: char, reading: &str, query: &str) -> usize {
    let mut score = 0;

    // Show written prefixes on top
    if query.len() > 0 && starts_with(reading, query) {
        score += 1000000;
    }

    // Show readings with more results first
    let index = k_reading::Engine::get_index(None);
    let score_qurey = format!("{}{}", literal, reading);
    if let Some(term_id) = index.dict().get_id(&score_qurey) {
        let posting = index.postings(0).unwrap().get_posting(term_id);
        score += (posting.len() as f32).log(1.01).floor() as usize;
    }

    score
}

#[inline]
fn starts_with(word: &str, reading: &str) -> bool {
    word.replace(".", "").to_hiragana().starts_with(reading)
}
