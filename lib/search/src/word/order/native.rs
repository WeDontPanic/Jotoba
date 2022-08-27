use engine::relevance::{data::SortData, RelevanceEngine};
use japanese::JapaneseExt;
use ngindex2::{item::IndexItem, termset::TermSet};
use types::jotoba::words::Word;

pub struct NativeOrder {
    orig_query: String,
}

impl NativeOrder {
    pub fn new(orig_query: String) -> Self {
        Self { orig_query }
    }
}

impl RelevanceEngine for NativeOrder {
    type OutItem = &'static Word;
    type IndexItem = IndexItem<u32>;
    type Query = TermSet;

    fn score<'item, 'query>(
        &mut self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let mut score = item.index_item().dice(item.query());

        let word = item.item();
        let query_str = japanese::to_halfwidth(item.query_str());

        let reading = japanese::to_halfwidth(&word.get_reading().reading);
        let reading_len = utils::real_string_len(&reading);
        let kana = japanese::to_halfwidth(&word.reading.kana.reading);

        if !reading.starts_with(&query_str) {
            //score += 4.0;
            score *= 0.99;
        }

        if self.orig_query != reading && self.orig_query != kana {
            //score += 100000000.0;
            score *= 0.6;
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
            score *= 0.9;
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
