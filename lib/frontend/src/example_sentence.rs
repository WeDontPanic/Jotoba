use japanese::furigana::SentencePartRef;
use types::jotoba::{languages::Language, sentences::Sentence, words::sense::Sense};

/// Returns the example sentence of a sense if available
#[inline]
fn get_ext_sentence(sense: &Sense) -> Option<&'static Sentence> {
    resources::get().sentences().by_id(sense.example_sentence?)
}

pub fn ext_sentence(
    sense: &Sense,
    language: &Language,
) -> Option<(Vec<SentencePartRef<'static>>, &'static str)> {
    let sentence = get_ext_sentence(sense)?;

    let translation = sentence
        .get_translations(*language)
        .or_else(|| sentence.get_translations(Language::English))?;

    let furigana = japanese::furigana::from_str(&sentence.furigana).collect::<Vec<_>>();

    Some((furigana, translation))
}
