use japanese::furigana::SentencePartRef;
use types::jotoba::{
    languages::Language,
    words::{filter_languages, sense::Sense, Word},
};

use crate::unescaped::UnescapedString;

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
pub fn unescaped_string<T: ToString>(s: T) -> UnescapedString {
    UnescapedString::new(s)
}

/// Returns the transive verion of `word`
#[inline]
pub fn get_transitive_counterpart(word: &Word) -> Option<Word> {
    let seq_id = word.transive_verion.as_ref()?;
    resources::get().words().by_sequence(*seq_id).cloned()
}

/// Returns the intransive verion of `word`
#[inline]
pub fn get_intransitive_counterpart(word: &Word) -> Option<Word> {
    let seq_id = word.intransive_verion.as_ref()?;
    resources::get().words().by_sequence(*seq_id).cloned()
}

/// Returns an example sentences of a `sense` if existing.
/// tries to use a sentence written in `language` or falls back to english
pub fn ext_sentence(
    sense: &Sense,
    language: &Language,
) -> Option<(Vec<SentencePartRef<'static>>, &'static str)> {
    let sentence = resources::get()
        .sentences()
        .by_id(sense.example_sentence?)?;

    let translation = sentence
        .get_translations(*language)
        .or_else(|| sentence.get_translations(Language::English))?;

    let furigana = japanese::furigana::from_str(&sentence.furigana).collect::<Vec<_>>();

    Some((furigana, translation))
}
