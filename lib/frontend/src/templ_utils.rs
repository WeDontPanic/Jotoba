use types::jotoba::{
    languages::Language,
    words::{filter_languages, Word},
};

use crate::BaseData;

pub fn need_3dot(data: &BaseData, word: &Word) -> bool {
    word.get_inflections().is_some()
        || word.collocations.is_some()
        || get_intransitive_counterpart(word).is_some()
        || get_transitive_counterpart(word).is_some()
        || (word.has_sentence(data.user_settings.user_lang)
            || (data.user_settings.show_english && word.has_sentence(Language::English)))
        || data.config.is_debug()
        || word.audio_file("ogg").is_some()
}

/// Returns a list of all collocations of a word
pub fn get_collocations(
    word: &Word,
    language: Language,
    show_english: bool,
) -> Vec<(String, String)> {
    if !word.has_collocations() {
        return vec![];
    }

    let word_storage = resources::get().words();

    let mut words = word
        .collocations
        .as_ref()
        .unwrap()
        .iter()
        .filter_map(|i| word_storage.by_sequence(*i))
        .cloned()
        .collect::<Vec<_>>();

    filter_languages(words.iter_mut(), language, show_english);

    words
        .into_iter()
        .map(|word| {
            let senses: Vec<String> = word
                .get_senses()
                .into_iter()
                .flatten()
                .take(5)
                .map(|i| i.glosses)
                .flatten()
                .map(|i| i.gloss)
                .collect();

            let reading = word.reading.kanji.unwrap_or(word.reading.kana).reading;

            (reading, senses.join(", "))
        })
        .collect()
}

#[inline]
pub fn get_transitive_counterpart(word: &Word) -> Option<Word> {
    let seq_id = word.transive_verion.as_ref()?;
    resources::get().words().by_sequence(*seq_id).cloned()
}

#[inline]
pub fn get_intransitive_counterpart(word: &Word) -> Option<Word> {
    let seq_id = word.intransive_verion.as_ref()?;
    resources::get().words().by_sequence(*seq_id).cloned()
}
