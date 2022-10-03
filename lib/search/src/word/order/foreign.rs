use std::{fs::File, io::BufReader, time::Instant};

use engine::relevance::{data::SortData, RelevanceEngine};
use once_cell::sync::{Lazy, OnceCell};
use types::jotoba::{
    languages::Language,
    words::{sense::Sense, Word},
};
use vsm::{doc_vec::DocVector, Vector};

use super::{ngindex::NgIndex, REMOVE_PARENTHESES};

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

    /* pub fn score2(&self, item: SortData<WordOutput, Vector, Vector>, user_lang: Language) -> f32 {
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
    } */
}

static TEST_INDEX: OnceCell<NgIndex> = OnceCell::new();

fn get_test_index() -> &'static NgIndex {
    TEST_INDEX.get_or_init(|| {
        bincode::deserialize_from(BufReader::new(
            File::open("/home/jojii/programming/rust/new_fg_search/ngindex").unwrap(),
        ))
        .unwrap()
    })
}

pub struct ForeignOrder2;

impl RelevanceEngine for ForeignOrder2 {
    type OutItem = &'static Word;
    type IndexItem = DocVector<u32>;
    type Query = Vector;

    #[inline]
    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let query = item.query();
        let index_item = item.index_item().vec();
        let gloss_sim = query.scalar(index_item);
        let word = item.item();

        let q_vec = get_test_index().build_vec(&item.query_str()).unwrap();

        //REMOVE_PARENTHESES.relpc
        let text_sim = word
            .gloss_iter_by_lang(Language::English, false)
            .map(|i| {
                let fmt = REMOVE_PARENTHESES.replace_all(i, "").to_lowercase();
                if fmt.trim().is_empty() {
                    return 0.0;
                }
                let vec = get_test_index().build_vec(&fmt).unwrap();
                let s = sim(&vec, &q_vec);
                s
            })
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap_or(0.0);

        let mut rel_add = 0.0;
        if text_sim >= 0.6 {
            rel_add += gloss_sim;
        }

        let score = (rel_add + text_sim) / 2.0;
        println!("{}: {score} ({gloss_sim})", word.get_reading_str());
        score
    }
}

#[inline]
fn sim(a: &Vector, b: &Vector) -> f32 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let both = a.overlapping(b).map(|(_, a_w, b_w)| a_w + b_w).sum::<f32>();

    let sum = a
        .sparse()
        .iter()
        .map(|i| i.1)
        .chain(b.sparse().iter().map(|i| i.1))
        .sum::<f32>();

    both / sum
}
