use super::kanji;
use crate::query::Query;
use japanese::JapaneseExt;
use types::jotoba::sentences::Sentence;

pub(crate) fn filter_sentence(query: &Query, sentence: &Sentence) -> bool {
    let lang = query.settings.user_lang;
    let show_english = query.settings.show_english;

    if sentence.get_translation(lang, show_english).is_none() {
        return false;
    }

    if query.form.is_kanji_reading() {
        let kreading = query
            .form
            .as_kanji_reading()
            .and_then(|i| kanji::get_reading(i))
            .unwrap();
        return kanji::sentence_matches(sentence, &kreading);
    }

    /*
        TODO
    if !query.must_contain.is_empty() {
        if !by_quot_marks(query, sentence) {
            return false;
        }
    }
    */

    true
}

fn by_quot_marks(query: &Query, sentence: &Sentence) -> bool {
    if !by_quot_marks_jp(query, sentence) {
        return false;
    }

    sentence
        .get_translation(query.lang(), query.show_english())
        .map(|sentence| by_quot_marks_fe(query, sentence))
        .unwrap_or(true)
}

fn by_quot_marks_fe(query: &Query, sentence: &str) -> bool {
    let sentence = sentence.to_lowercase();
    let iter = query.must_contain.iter().filter(|i| !i.is_japanese());

    for needle in iter {
        if !sentence.contains(needle) {
            return false;
        }
    }

    true
}

fn by_quot_marks_jp(query: &Query, sentence: &Sentence) -> bool {
    let iter = query.must_contain.iter().filter(|i| i.is_japanese());

    let jp_sentence = &sentence.japanese;

    for needle in iter {
        let is_kana = needle.is_kana();

        // If kana reading and kana contains needle
        if (is_kana && sentence.get_kana().contains(needle))
            // Or full reading contains
            || (!is_kana && jp_sentence.contains(needle))
        {
            continue;
        }

        return false;
    }

    true
}
