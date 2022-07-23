use crate::query::{Query, QueryLang};
use std::borrow::Borrow;
use types::jotoba::words::Word;

pub struct WordFilter {
    query: Query,
    jlpt_lvl: Option<u8>,
}

impl WordFilter {
    pub fn new(query: Query) -> Self {
        let jlpt_lvl = query.tags.iter().find_map(|i| i.as_jlpt());
        Self { query, jlpt_lvl }
    }

    /// Returns `true` for all words the query has a filter for aka if the word should be filtered out of the results
    #[inline]
    pub fn filter_word<W: Borrow<Word>>(&self, word: W) -> bool {
        #[inline]
        fn inner(wf: &WordFilter, word: &Word) -> Option<()> {
            wf.by_misc_tags(word)?;
            wf.by_language(word)?;
            wf.by_pos_tags(word)?;
            wf.by_jlpt(word)?;

            if wf.query.q_lang == QueryLang::Foreign {
                wf.by_quot_marks(word)?;
            }

            Some(())
        }

        inner(self, word.borrow()).is_none()
    }

    #[inline]
    fn by_language(&self, w: &Word) -> Option<()> {
        w.has_language(self.query.get_search_lang(), self.query.show_english())
            .then(|| ())
    }

    #[inline]
    fn by_jlpt(&self, w: &Word) -> Option<()> {
        // Ignore if not set
        if self.jlpt_lvl.is_none() {
            return Some(());
        }

        (w.get_jlpt_lvl() == self.jlpt_lvl).then(|| ())
    }

    #[inline]
    fn by_pos_tags(&self, w: &Word) -> Option<()> {
        w.has_all_pos_iter(self.query.get_part_of_speech_tags())
            .then(|| ())
    }

    #[inline]
    fn by_misc_tags(&self, w: &Word) -> Option<()> {
        self.query
            .get_misc_tags()
            .all(|mt| w.has_misc(mt))
            .then(|| ())
    }

    fn by_quot_marks(&self, w: &Word) -> Option<()> {
        if self.query.must_contain.is_empty() {
            return Some(());
        }

        let mut quot_terms = self.query.must_contain.clone();

        for i in w.gloss_iter_by_lang(self.query.get_search_lang(), self.query.show_english()) {
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
}
