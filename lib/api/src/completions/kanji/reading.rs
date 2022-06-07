use std::cmp::Ordering;
use types::api::completions::{Response, WordPair};
use types::{api::completions::SuggestionType, jotoba::kanji};

/// Gets suggestions for kanji reading search eg: "痛 いた.い"
pub fn suggestions(kanji_reading: kanji::reading::ReadingSearch) -> Option<Response> {
    let kanji_storage = resources::get().kanji();

    let literal = kanji_reading.literal;
    let reading = kanji_reading.reading.replace("。", "").replace(".", "");

    let character = kanji_storage.by_literal(literal)?;

    let mut readings = character
        .kunyomi
        .iter()
        .chain(character.onyomi.iter())
        .map(|i| WordPair {
            primary: i.clone(),
            secondary: Some(literal.to_string()),
        })
        .collect::<Vec<_>>();

    if readings.is_empty() {
        return None;
    }

    readings.sort_by(|a, b| order(a, b, &reading));

    Some(Response::with_type(readings, SuggestionType::KanjiReading))
}

fn order(a: &WordPair, b: &WordPair, reading: &str) -> Ordering {
    utils::bool_ord(starts_with(a, reading), starts_with(b, reading))
}

fn starts_with(word: &WordPair, reading: &str) -> bool {
    word.primary.replace(".", "").starts_with(reading)
}
