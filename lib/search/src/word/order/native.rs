use engine::relevance::{data::SortData, RelevanceEngine};
use indexes::ng_freq::{term_dist, NgFreqIndex};
use japanese::ToKanaExt;
use jp_utils::JapaneseExt;
use ngindex::{item::IndexItem, termset::TermSet};
use sparse_vec::{SpVec32, VecExt};
use types::jotoba::words::Word;

pub struct NativeOrder {
    orig_query: String,
    orig_query_ts: Option<TermSet>,

    query_hw: String,

    /// Word index in sentence reader
    w_index: Option<usize>,

    query_vec: SpVec32,
}

impl NativeOrder {
    #[inline]
    pub fn new(orig_query: String) -> Self {
        Self {
            orig_query,
            orig_query_ts: None,
            w_index: None,
            query_vec: SpVec32::empty(),
            query_hw: String::new(),
        }
    }

    /// Set a custom sentence reader word index
    pub fn with_w_index(mut self, index: usize) -> Self {
        self.w_index = Some(index);
        self
    }

    pub fn with_oquery_ts(mut self, ts: TermSet) -> Self {
        self.orig_query_ts = Some(ts);
        self
    }

    #[inline]
    fn exceeded_threshold<'i, 'q, A, B, C>(item: &SortData<'i, 'q, A, B, C>, score: f32) -> bool {
        item.threshold().map(|th| score < th).unwrap_or(false)
    }

    #[inline]
    fn text_sim(&self, word: &Word) -> f32 {
        word.reading_iter(true)
            .map(|i| self.reading_sim(&i.reading.to_halfwidth().to_hiragana()))
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(0.0)
    }

    #[inline]
    fn reading_sim(&self, reading: &str) -> f32 {
        let vec = build_ng_vec(reading);
        term_dist(&vec, &self.query_vec)
    }
}

impl RelevanceEngine for NativeOrder {
    type OutItem = &'static Word;
    type IndexItem = IndexItem<u32>;
    type Query = TermSet;

    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let word = item.item();
        let mut score = item.index_item().dice(item.query());

        // If alternative reading matches query exactly
        if Self::exceeded_threshold(item, score) {
            return 0.0;
        }

        score *= self.text_sim(word);

        if let Some(ref o_ts) = self.orig_query_ts {
            if self.w_index.unwrap_or(0) == 0 {
                let new = item.index_item().dice(o_ts);
                if new > score {
                    score = new;
                } else {
                    score *= 0.7;
                }
            }
        }

        if Self::exceeded_threshold(item, score) {
            return 0.0;
        }

        let kana = word.reading.kana.reading.to_halfwidth().to_hiragana();

        // Words with query as substring have more relevance
        // スイス: スイス人 > スパイス
        if !kana.contains(&self.query_hw) {
            //score *= 0.8;
        }

        if Self::exceeded_threshold(item, score) {
            return 0.0;
        }

        if kana != self.orig_query && word.get_reading().reading.to_halfwidth() != self.orig_query {
            score *= 0.7;
        }

        if Self::exceeded_threshold(item, score) {
            return 0.0;
        }

        if word.jlpt_lvl.is_none() {
            score *= 0.999;
        }

        // Is common
        if !word.is_common() {
            score *= 0.999;
        }

        //let reading_len = utils::real_string_len(&reading);
        /* if reading_len == 1 && reading.is_kanji() {
            let kanji = reading.chars().next().unwrap();
            let norm = indexes::get()
                .kanji()
                .reading_freq()
                .norm_reading_freq(kanji, word.get_kana());
            if let Some(_read_freq) = norm {
                //score += read_freq;
            }
        } */

        score
    }

    fn init(&mut self, init: engine::relevance::RelEngineInit) {
        self.query_vec = build_ng_vec(&init.query.to_halfwidth().to_hiragana());
        self.query_hw = init.query.to_halfwidth().to_hiragana();
    }
}

#[inline]
fn ng_freq_index() -> &'static NgFreqIndex {
    indexes::get().word().native().tf_index()
}

#[inline]
fn build_ng_vec(term: &str) -> SpVec32 {
    ng_freq_index().build_custom_vec(term, |freq, tot| (tot / freq).log2())
}
