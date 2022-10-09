use engine::relevance::RelevanceEngine;
use types::jotoba::words::Word;

pub struct KanjiReadingRelevance;

impl RelevanceEngine for KanjiReadingRelevance {
    type OutItem = &'static Word;
    type IndexItem = u32;
    type Query = String;

    #[inline]
    fn score<'item, 'query>(
        &self,
        item: &engine::relevance::data::SortData<
            'item,
            'query,
            Self::OutItem,
            Self::IndexItem,
            Self::Query,
        >,
    ) -> f32 {
        let word = item.item();
        let mut score: f32 = 0.0;

        if word.is_common() {
            score += 100.0;
        }

        if let Some(jlpt) = word.get_jlpt_lvl() {
            score += jlpt as f32 * 10.0;
        }

        if score == 0.0 {
            // Show shorter words on top if they aren't important
            let reading_len = word.reading.get_reading().reading.chars().count();
            //score = 100usize.saturating_sub(reading_len * 2);
            score = (0f32).max(100.0 - reading_len as f32 * 2.0);
        } else {
            score += 100.0;
        }

        score
    }
}
