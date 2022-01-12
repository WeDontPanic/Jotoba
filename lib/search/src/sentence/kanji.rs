use japanese::{jp_parsing::JA_NL_PARSER, JapaneseExt};
use types::jotoba::{
    kanji::{Reading, ReadingSearch},
    sentences::Sentence,
};

pub(crate) fn sentence_matches(sentence: &Sentence, reading: &Reading) -> bool {
    let lit = reading.get_lit_str();

    if reading.is_full_reading() {
        let parsed_furi = japanese::furigana::from_str(&sentence.furigana);
        let reading_hira = reading.get_raw().to_hiragana();

        for i in parsed_furi {
            if i.kanji.is_none() {
                continue;
            }

            let curr_kanji = i.kanji.unwrap();
            if !curr_kanji.contains(&lit) {
                continue;
            }

            if i.kana.to_hiragana().contains(&reading_hira) {
                return true;
            }
        }

        return false;
    }

    // Kunyomi

    let formatted = reading.format_reading_with_literal();
    for morph in JA_NL_PARSER.parse(&sentence.japanese) {
        let reading = morph.lexeme;
        if reading == formatted {
            return true;
        }
    }

    false
}

pub(crate) fn get_reading(reading: &ReadingSearch) -> Option<Reading> {
    let kanji_storage = resources::get().kanji();
    let kanji = kanji_storage.by_literal(reading.literal)?;
    let reading = kanji.find_reading(&reading.reading)?;
    Some(reading)
}
