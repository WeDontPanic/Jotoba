use super::kanji;
use crate::{engine, query::Query};
use index_framework::traits::{backend::Backend, dictionary::IndexDictionary};
use jp_utils::JapaneseExt;
use sparse_vec::VecExt;
use types::jotoba::sentences::Sentence;
use vsm::doc_vec::DocVector;

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

    if !query.must_contain.is_empty() {
        if !by_quot_marks(query, sentence) {
            return false;
        }
    }

    if !query
        .tags
        .iter()
        .filter_map(|i| i.as_sentence_tag())
        .all(|tag| sentence.has_tag(tag))
    {
        return false;
    }

    true
}

fn by_quot_marks(query: &Query, sentence: &Sentence) -> bool {
    if !by_quot_marks_jp(query, sentence) {
        return false;
    }

    // We're doing filtering for foreign words directly as search engine filter
    /*
    sentence
        .get_translation(query.lang(), query.show_english())
        .map(|sentence| by_quot_marks_fe(query, sentence))
        .unwrap_or(true)
        */
    true
}

/*
fn by_quot_marks_fe(query: &Query, sentence: &str) -> bool {
    let sentence = sentence.to_lowercase();
    let sentence: Vec<_> = sentence.split(' ').collect();

    let iter = query.must_contain.iter().filter(|i| !i.is_japanese());

    for needle in iter {
        if !sentence.contains(&needle.as_str()) {
            return false;
        }
    }

    true
}
*/

fn by_quot_marks_jp(query: &Query, sentence: &Sentence) -> bool {
    let jp_sentence = &sentence.japanese;

    let jp_terms = query.must_contain.iter().filter(|i| i.is_japanese());
    for needle in jp_terms {
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

/// Vector filter for Sentences filtering based on quoted terms
pub struct FeQotTermsVecFilter {
    mc_terms: Vec<u32>,
    filter_all: bool,
}

impl FeQotTermsVecFilter {
    pub fn new(query: &Query) -> Self {
        // If there is a term that is not indexed and thus can't be found,
        // filter out all results
        let mut filter_all = false;
        let mut mc_terms = vec![];

        let index = indexes::get().sentence().foreign();
        let ix_dict = index.dict();

        'o: for t in query.must_contain.iter().filter(|i| !i.is_japanese()) {
            for term in engine::sentences::foreign::all_terms(t).into_iter() {
                if let Some(v) = ix_dict.get_id(&term) {
                    mc_terms.push(v as u32);
                    continue;
                }

                filter_all = true;
                mc_terms.clear();
                break 'o;
            }
        }

        Self {
            mc_terms,
            filter_all,
        }
    }

    pub fn filter(&self, sentence: &DocVector<u32>) -> bool {
        if self.filter_all {
            return false;
        }

        if self.mc_terms.is_empty() {
            return true;
        }

        self.mc_terms
            .iter()
            .all(|dim| sentence.vec().has_dim(*dim as usize))
    }
}
