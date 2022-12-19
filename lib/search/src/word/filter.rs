use crate::query::Query;
use jp_utils::JapaneseExt;
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
            wf.by_katakana_tag(word)?;

            wf.by_quot_marks(word)?;

            Some(())
        }

        inner(self, word.borrow()).is_none()
    }

    #[inline]
    fn by_language(&self, w: &Word) -> Option<()> {
        w.has_language(self.query.lang_param()).then(|| ())
    }

    #[inline]
    fn by_katakana_tag(&self, w: &Word) -> Option<()> {
        let has_tag = self.query.has_tag(crate::query::Tag::Katakana);
        (!has_tag || w.get_reading_str().is_katakana()).then(|| ())
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

        let (jp_q_terms, mut fn_q_terms): (Vec<_>, Vec<_>) = self
            .query
            .must_contain
            .iter()
            .partition(|i| i.is_japanese());

        if !fn_q_terms.is_empty() {
            for i in w.gloss_iter_by_lang(self.query.lang_param()) {
                let i = i.to_lowercase();
                fn_q_terms.retain(|k| !i.contains(k.as_str()));
                if fn_q_terms.is_empty() {
                    break;
                }
            }
        }

        if !jp_q_terms.is_empty() {
            for term in jp_q_terms {
                self.by_quot_marks_jp(w, &term)?;
            }
        }

        // Success if all quted terms were removed
        fn_q_terms.is_empty().then(|| ())
    }

    #[inline]
    fn by_quot_marks_jp(&self, w: &Word, q_term: &str) -> Option<()> {
        if q_term.is_kana() {
            if !w.get_kana().contains(q_term) {
                return None;
            }
        } else if !w.reading_iter(false).any(|i| i.reading.contains(q_term)) {
            return None;
        }

        Some(())
    }
}
