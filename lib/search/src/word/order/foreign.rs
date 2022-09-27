use crate::engine::words::foreign::output::WordOutput;
use engine::relevance::data::SortData;
use indexes::relevance::RelevanceIndex;
use log::warn;
use spin::Mutex;
use std::collections::HashMap;
use types::jotoba::{languages::Language, words::sense::Sense};
use vector_space_model2::{term_store::TermIndexer, Vector};

pub struct ForeignOrder {
    query_vecs: Mutex<HashMap<(String, Language), Option<Vector>>>,
}

impl ForeignOrder {
    pub fn new() -> Self {
        let query_vecs = Mutex::new(HashMap::with_capacity(1));
        Self { query_vecs }
    }

    #[inline]
    fn vec_cached(&self, query: &String, language: Language) -> Option<Option<Vector>> {
        let lock = self.query_vecs.lock();
        let vec = lock.get(&(query.clone(), language))?;
        Some(vec.clone())
    }

    #[inline]
    fn new_vec_cached(&self, query: &String, language: Language) -> Option<Vector> {
        if let Some(vec) = self.vec_cached(query, language) {
            return vec;
        }
        let index = indexes::get().word().foreign(language)?;
        let indexer = index.get_indexer();
        let vec = make_search_vec(&indexer, query);
        let mut lock = self.query_vecs.lock();
        lock.insert((query.to_string(), language), vec.clone());
        vec
    }

    #[inline]
    fn gloss_relevance(
        &self,
        query_str: &String,
        seq_id: u32,
        sense: &Sense,
        sg_id: u16,
    ) -> Option<f32> {
        let rel_index = indexes::get().word().relevance(sense.language)?;
        let rel_vec = rel_index.get(seq_id, sg_id)?;
        let query_vec = self.new_vec_cached(query_str, sense.language)?;
        Some(vec_similarity(rel_vec, &query_vec, &rel_index))
    }

    pub fn score(&self, item: SortData<WordOutput, Vector, Vector>, user_lang: Language) -> f32 {
        let txt_rel = item.vec_similarity();

        let word_output = item.item();
        let query_lang = item.language().unwrap();
        let query_str = item.query_str().trim().to_lowercase();

        let word = word_output.word;

        let txt_dist = word_output
            .position_iter()
            .map(|i| {
                let gloss = word.get_sense_gloss(i.2).unwrap();
                let gloss = gloss.1.gloss.to_lowercase();
                let l = levenshtein::levenshtein(&gloss, &item.query_str().to_lowercase());
                let max_len = gloss.len().max(item.query_str().len());
                let sim = 1.0 - (l as f32 / max_len as f32);
                (sim, gloss)
            })
            .max_by(|a, b| a.0.total_cmp(&b.0))
            .unwrap();

        let txt_dist = txt_dist.0;

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
                self.gloss_relevance(&query_str, word.sequence, sense, sg_id)
                    .map(|i| i as f32 * multilpier)
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_else(|| {
                /*
                super::foreign_search_fall_back(word, txt_rel, &query_str, query_lang, user_lang)
                    as f32
                    * txt_rel
                    * 0.00
                */
                0.0
            });

        if word.get_reading_str() == "音楽"
            || word.get_reading_str() == "ミュージック"
            || word.get_reading_str() == "鳴り物入り"
        {
            //println!("{item:#?}");
            println!("{}", word.get_reading_str());
            println!("gloss relevance: {gloss_relevance}");
            println!("text_score relevance: {txt_dist}");
            let total = gloss_relevance + txt_dist * 10.0;
            println!("{total}");
            println!("------------",);
        }

        //println!("gloss relevance: {gloss_relevance}");
        // println!("text_score relevance: {txt_dist}");

        let mut rel_add = 0.0;
        if txt_dist > 0.8 {
            rel_add += gloss_relevance;
            //println!("rel_add: {rel_add}");
        } else {
            //rel_add += gloss_relevance * 0.000001;
        }

        let total = rel_add + txt_dist * 10.0;
        // println!("{total}");
        total
    }
}

fn make_search_vec(indexer: &TermIndexer, query: &str) -> Option<Vector> {
    let terms: Vec<_> = query
        .split(' ')
        .filter_map(|s_term| Some((s_term, indexer.get_term(s_term)?)))
        .map(|(_, dim)| (dim as u32, 1.0))
        .collect();

    if terms.is_empty() {
        return None;
    }

    Some(Vector::create_new_raw(terms))
}

#[inline]
fn vec_similarity(src_vec: &Vector, query: &Vector, r_index: &RelevanceIndex) -> f32 {
    return src_vec.get_dim(0).unwrap_or(0.0);
    let mut sum = 0.0;
    let mut overlapping_count: usize = 0;

    for i in src_vec.overlapping(query) {
        sum += i.1;
        overlapping_count += 1;
    }

    let query_imp = important_count(&query, r_index);
    let src_imp = important_count(&src_vec, r_index);

    let diff = (query_imp.abs_diff(src_imp) + 1) as f32;
    let important_mult = (1.0 / diff) + 1.0;

    let src_len = src_vec.sparse_vec().len();
    let query_len = query.sparse_vec().len();
    let mut vec_len_mult = 1.0; //src_len.min(query_len) as f32 / query_len as f32;
    if src_len < query_len {
        vec_len_mult = src_len as f32 / query_len as f32;
    }

    //let v = query.sparse_vec().len() as f32 / overlapping_count as f32;

    (overlapping_count as f32 * important_mult * vec_len_mult * 100.0) + sum * vec_len_mult * 100.0
}

#[inline]
fn important_count(vec: &Vector, r_index: &RelevanceIndex) -> usize {
    vec.sparse_vec()
        .iter()
        .filter(|(dim, _)| r_index.is_important(*dim))
        .count()
}
