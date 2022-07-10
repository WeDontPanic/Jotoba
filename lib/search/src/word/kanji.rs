use itertools::Itertools;
use japanese::CharType;
use types::jotoba::{kanji::Kanji, words::Word};

/// Retrieves all (up to 10) kanji for words in correct order without duplicates
pub fn load_word_kanji_info(words: &[Word]) -> Vec<Kanji> {
    let kanji_resources = resources::get().kanji();
    words
        .iter()
        .filter_map(|i| {
            let kanji = &i.reading.kanji.as_ref()?.reading;
            Some(japanese::all_words_with_ct(kanji, CharType::Kanji))
        })
        .flatten()
        .map(|i| i.chars().collect::<Vec<_>>())
        .flatten()
        .filter_map(|i| kanji_resources.by_literal(i).cloned())
        .unique_by(|i| i.literal)
        .take(10)
        .collect()
}
