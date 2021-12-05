use japanese::furigana::SentencePartRef;
use resources::{models::words::Sense, types::jotoba::languages::Language};
use types::jotoba::sentences::Sentence;

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

    /*
     * TODO: aaaa
    let translation = sentence
        .get_translations(*language)
        .or_else(|| sentence.get_translations(Language::English))?;
        */

    let furigana = japanese::furigana::from_str(&sentence.furigana).collect::<Vec<_>>();
    let translation = "";

    Some((furigana, translation))
}
