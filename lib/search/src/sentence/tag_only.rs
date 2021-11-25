use error::Error;
use resources::parse::jmdict::languages::Language;

use crate::query::{Query, Tag};

use super::result::SentenceResult;

pub(super) fn search(query: &Query) -> Result<SentenceResult, Error> {
    let filter_tag = query
        .tags
        .iter()
        .find(|i| i.is_empty_allowed())
        // We expect to find one since this function should only be called if there is one
        .ok_or(Error::Unexpected)?;

    if let Tag::Jlpt(jlpt) = filter_tag {
        return jlpt_search(query, *jlpt);
    } else {
        return Ok(SentenceResult::default());
    }
}

fn jlpt_search(query: &Query, jlpt: u8) -> Result<SentenceResult, Error> {
    assert!(jlpt > 0 && jlpt < 6);

    let resources = resources::get();

    let sentence_jlpt = match resources.sentence_jlpt(jlpt) {
        Some(sentence_jlpt) => sentence_jlpt,
        None => return Ok(SentenceResult::default()),
    };

    let senences = sentence_jlpt
        .filter(|sentence| {
            sentence.has_translation(query.settings.user_lang)
                && (sentence.has_translation(Language::English) && query.settings.show_english)
        })
        .take(10000)
        .collect::<Vec<_>>();

    let len = senences.len();

    let sentences = senences
        .into_iter()
        .skip(query.page_offset)
        .take(query.settings.page_size as usize)
        .filter_map(|i| super::map_sentence_to_item(i, query.settings.user_lang, query))
        .collect::<Vec<_>>();

    let hidden = query.has_tag(Tag::Hidden);
    Ok(SentenceResult {
        items: sentences,
        len,
        hidden,
    })
}
