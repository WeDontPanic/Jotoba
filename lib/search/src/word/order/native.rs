use engine::relevance::{data::SortData, RelevanceEngine};
use indexes::ng_freq::{term_dist, NgFreqIndex};
use japanese::JapaneseExt;
use ngindex::{item::IndexItem, termset::TermSet};
use sparse_vec::{SpVec32, VecExt};
use types::jotoba::words::Word;

pub struct NativeOrder {
    _orig_query: String,
    orig_query_ts: Option<TermSet>,

    /// Word index in sentence reader
    w_index: Option<usize>,

    query_vec: SpVec32,
}

impl NativeOrder {
    #[inline]
    pub fn new(orig_query: String) -> Self {
        Self {
            _orig_query: orig_query,
            orig_query_ts: None,
            w_index: None,
            query_vec: SpVec32::empty(),
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

        let reading_vec = if item.query_str().has_kanji() {
            build_ng_vec(word.get_reading_str())
        } else {
            word.get_kana();
            build_ng_vec(word.get_kana())
        };
        score *= term_dist(&reading_vec, &self.query_vec);

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

        let query_str = japanese::to_halfwidth(item.query_str()).to_hiragana();

        let reading = japanese::to_halfwidth(&word.get_reading().reading);
        let reading_len = utils::real_string_len(&reading);
        let kana = japanese::to_halfwidth(&word.reading.kana.reading).to_hiragana();

        // Words with query as substring have more relevance
        // スイス: スイス人 > スパイス
        if !kana.contains(&query_str) {
            score *= 0.8;
        }

        if kana == self._orig_query || reading == self._orig_query {
            score = (score * 10.0).min(1.0);
        }

        if word.jlpt_lvl.is_none() {
            score *= 0.999;
        }

        // Is common
        if !word.is_common() {
            //score += 3.0;
            score *= 0.999;
        }

        if !reading.starts_with(&query_str)
            && !(query_str.is_kana() && reading.starts_with(&query_str))
        {
            //score += 200.0;
            //score *= 0.9;
        }

        if reading_len == 1 && reading.is_kanji() {
            let kanji = reading.chars().next().unwrap();
            let norm = indexes::get()
                .kanji()
                .reading_freq()
                .norm_reading_freq(kanji, word.get_kana());
            if let Some(_read_freq) = norm {
                //score += read_freq;
            }
        }

        // If alternative reading matches query exactly
        if word
            .reading
            .alternative
            .iter()
            .map(|i| japanese::to_halfwidth(&i.reading))
            .any(|i| i == *query_str)
        {
            //score += 60.0;
            score *= 0.8;
        }

        //println!("{}: {}", word.get_reading_str(), score);
        score
    }

    fn init(&mut self, init: engine::relevance::RelEngineInit) {
        self.query_vec = build_ng_vec(&init.query);
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
