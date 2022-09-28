use crate::engine::words::foreign::output::WordOutput;
use engine::relevance::data::SortData;
use types::jotoba::{languages::Language, words::sense::Sense};
use vector_space_model2::Vector;

pub struct ForeignOrder;

impl ForeignOrder {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    #[inline]
    fn gloss_relevance(&self, seq_id: u32, sense: &Sense, sg_id: u16) -> Option<f32> {
        indexes::get()
            .word()
            .relevance(sense.language)?
            .get(seq_id, sg_id)?
            .get_dim(0)
    }

    pub fn score(&self, item: SortData<WordOutput, Vector, Vector>, user_lang: Language) -> f32 {
        let word_output = item.item();

        let query_lang = item.language().unwrap();
        let word = word_output.word;

        let txt_dist = word_output
            .position_iter()
            .map(|i| {
                let gloss = word.get_sense_gloss(i.2).unwrap();
                let gloss = gloss.1.gloss.to_lowercase();
                let l = levenshtein::levenshtein(&gloss, &item.query_str().to_lowercase());
                let max_len = gloss.len().max(item.query_str().len());
                1.0 - (l as f32 / max_len as f32)
            })
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap_or(0.0);

        /* println!(
            "------------\n{:?} ({})",
            word.get_reading().reading,
            word.sequence
        ); */

        let gloss_relevance = word_output
            .position_iter()
            .filter_map(|(s_id, _, sg_id)| {
                let sense = word.sense_by_id(s_id).expect("Failed to get sense");
                let mut multiplier = 1.0;
                if sense.language != user_lang {
                    multiplier = 0.1;
                }
                Some((sense, sg_id, multiplier))
            })
            .filter_map(|(sense, sg_id, multilpier)| {
                self.gloss_relevance(word.sequence, sense, sg_id)
                    .map(|i| i * multilpier)
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_else(|| {
                super::foreign_search_fall_back(
                    word,
                    txt_dist,
                    item.query_str(),
                    query_lang,
                    user_lang,
                ) as f32
                    * 0.001
            });

        // println!("gloss relevance: {gloss_relevance}");
        // println!("text_score relevance: {txt_dist}");

        let mut rel_add = 0.0;
        if txt_dist > 0.8 {
            rel_add += gloss_relevance;
        }

        (rel_add + txt_dist * 10.0) / 10.0
    }
}
