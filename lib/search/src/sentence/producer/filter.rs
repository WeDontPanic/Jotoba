use super::kanji;
use crate::query::Query;
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

    true
}
