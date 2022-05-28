use crate::engine::{self, words::foreign::output::WordOutput};
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
        let index = engine::words::foreign::index::get(language)?;
        let indexer = index.get_indexer().clone();
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
    ) -> Option<usize> {
        let rel_index = engine::words::foreign::index::RELEVANCE_INDEXES
            .get()?
            .get(&sense.language)?;
        let rel_vec = rel_index.get(&(seq_id, sg_id))?;
        let query_vec = self.new_vec_cached(query_str, sense.language)?;
        Some((overlapping_vals(rel_vec, &query_vec) * 1000.0) as usize)
    }

    pub fn score(
        &self,
        word_output: &WordOutput,
        relevance: f32,
        query_str: &str,
        query_lang: Language,
        user_lang: Language,
    ) -> usize {
        let query_str = query_str.trim().to_lowercase();

        let text_score = (relevance as f64 * 10.0) as usize;
        let word = word_output.word;

        let gloss_relevance = word_output
            .position_iter()
            .filter_map(|(s_id, _, sg_id)| {
                let sense = word.sense_by_id(s_id).expect("Failed to get sense");
                let mut multiplier = 1.0;
                if sense.language == user_lang {
                    multiplier = 2.0;
                }
                Some((sense, sg_id, multiplier))
            })
            .filter_map(|(sense, sg_id, multilpier)| {
                //println!("------------\n{:?}", word.get_reading().reading);
                self.gloss_relevance(&query_str, word.sequence, sense, sg_id)
                    .map(|i| (i as f32 * multilpier) as usize)
            })
            .map(|i| i + 1000)
            .max()
            .unwrap_or_else(|| {
                super::foreign_search_fall_back(word, relevance, &query_str, query_lang, user_lang)
            });

        //println!("gloss relevance: {gloss_relevance}");
        //println!("text_score relevance: {text_score}");
        gloss_relevance + text_score
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
fn overlapping_vals(src_vec: &Vector, query: &Vector) -> f32 {
    let mut sum = 0.0;
    let mut overlapping_count = 0;

    for i in src_vec.overlapping(query) {
        sum += i.1;
        overlapping_count += 1;
    }

    //println!("overlapping_count: {overlapping_count}");
    //println!("sum: {sum}");
    (overlapping_count as f32 * 100.0) + sum
}
