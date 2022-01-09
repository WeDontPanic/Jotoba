use japanese::JapaneseExt;
use types::jotoba::{
    kanji::{Kanji, Reading},
    sentences::Sentence,
};

pub(crate) fn sentence_matches(sentence: &Sentence, kanji: &Kanji, reading: &str) -> bool {
    let lit = kanji.literal.to_string();

    let parsed_furi = japanese::furigana::from_str(&sentence.furigana);

    for i in parsed_furi {
        if i.kanji.is_none() {
            continue;
        }

        let curr_kanji = i.kanji.unwrap();
        if !curr_kanji.contains(&lit) {
            continue;
        }

        if i.kana.to_hiragana().contains(reading) {
            return true;
        }
    }

    false
}

pub(crate) fn get_reading(reading: &Reading) -> Option<(Kanji, String)> {
    let kanji_storage = resources::get().kanji();

    let kanji = kanji_storage.by_literal(reading.literal)?;

    if !kanji.has_reading(&reading.reading) {
        return None;
    }

    let k_reading = reading.reading.to_hiragana();

    Some((kanji.to_owned(), k_reading))
}
