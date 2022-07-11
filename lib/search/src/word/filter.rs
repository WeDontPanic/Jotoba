use crate::query::{Query, QueryLang};
use std::borrow::Borrow;
use types::jotoba::words::Word;

/// Returns `true` for all words the query has a filter for aka if the word should be filtered out of the results
#[inline]
pub fn filter_word<W: Borrow<Word>>(word: W, query: &Query) -> bool {
    #[inline]
    fn inner(word: &Word, query: &Query) -> Option<()> {
        by_language(word, query)?;
        by_pos_tags(word, query)?;
        by_misc_tags(word, query)?;

        if query.q_lang == QueryLang::Foreign {
            by_quot_marks(word, query)?;
        }

        Some(())
    }

    inner(word.borrow(), query).is_none()
}

#[inline]
fn by_language(w: &Word, query: &Query) -> Option<()> {
    w.has_language(query.get_search_lang(), query.show_english())
        .then(|| ())
}

#[inline]
fn by_pos_tags(w: &Word, query: &Query) -> Option<()> {
    w.has_all_pos_iter(query.get_part_of_speech_tags())
        .then(|| ())
}

#[inline]
fn by_misc_tags(w: &Word, query: &Query) -> Option<()> {
    query.get_misc_tags().all(|mt| w.has_misc(mt)).then(|| ())
}

fn by_quot_marks(w: &Word, query: &Query) -> Option<()> {
    if query.must_contain.is_empty() {
        return Some(());
    }

    let mut quot_terms = query.must_contain.clone();

    for i in w.gloss_iter_by_lang(query.get_search_lang(), query.show_english()) {
        let i = i.to_lowercase();
        quot_terms.retain(|j| !i.contains(j));
        if quot_terms.is_empty() {
            break;
        }
    }

    if !quot_terms.is_empty() {
        for i in w.reading_iter(true) {
            let i = &i.reading;
            quot_terms.retain(|j| !i.contains(j));
            if quot_terms.is_empty() {
                break;
            }
        }
    }

    // Success if all quted terms were removed
    quot_terms.is_empty().then(|| ())
}
