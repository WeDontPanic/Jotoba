use engine::relevance::{data::SortData, RelevanceEngine};
use japanese::JapaneseExt;
use ngindex2::{item::IndexItem, termset::TermSet};
use types::jotoba::words::Word;

pub struct NativeOrder {
    _orig_query: String,
    orig_query_ts: Option<TermSet>,

    /// Word index in sentence reader
    w_index: Option<usize>,
}

impl NativeOrder {
    #[inline]
    pub fn new(orig_query: String) -> Self {
        Self {
            _orig_query: orig_query,
            orig_query_ts: None,
            w_index: None,
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
        let mut score = item.index_item().dice(item.query());

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

        let word = item.item();
        let query_str = japanese::to_halfwidth(item.query_str());

        let reading = japanese::to_halfwidth(&word.get_reading().reading);
        let reading_len = utils::real_string_len(&reading);
        let kana = japanese::to_halfwidth(&word.reading.kana.reading);

        if kana == self._orig_query || reading == self._orig_query {
            score = (score * 10.0).min(1.0);
        }

        if word.jlpt_lvl.is_none() {
            score *= 0.99;
        }

        // Is common
        if !word.is_common() {
            //score += 3.0;
            score *= 0.99;
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
                .reading_fre()
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

        score
    }
}
