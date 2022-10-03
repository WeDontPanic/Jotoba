use ngram_tools::iter::wordgrams::Wordgrams;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vsm::Vector;

#[derive(Serialize, Deserialize)]
pub struct NgIndex {
    freq: HashMap<String, u32>,
    ng_ids: HashMap<String, u32>,
    id_term: HashMap<u32, String>,
    n: usize,
    total: usize,
}

impl NgIndex {
    pub fn new(n: usize) -> Self {
        assert!(n > 0);
        Self {
            freq: HashMap::new(),
            ng_ids: HashMap::new(),
            id_term: HashMap::new(),
            n,
            total: 0,
        }
    }

    pub fn index_new(&mut self, gloss: &str) {
        let padded = self.get_paddded(gloss);
        let n = self.n_for(gloss);
        let ngrams = Wordgrams::new(&padded, n);

        for ngram in ngrams {
            self.insert(ngram.to_string());
        }
    }

    pub fn insert(&mut self, ng: String) {
        self.total += 1;
        if let Some(freq) = self.freq.get_mut(&ng) {
            *freq += 1;
            return;
        }

        let new_id = self.ng_ids.len() as u32;
        self.ng_ids.insert(ng.clone(), new_id);
        self.freq.insert(ng.clone(), 1);
        self.id_term.insert(new_id, ng);
    }

    #[inline]
    pub fn ng_by_id(&self, id: u32) -> Option<&str> {
        self.id_term.get(&id).map(|i| i.as_str())
    }

    #[inline]
    pub fn freq(&self, term: &str) -> Option<f32> {
        self.freq.get(term).map(|i| *i as f32)
    }

    #[inline]
    pub fn inverted_freq(&self, term: &str) -> Option<f32> {
        let freq = *self.freq.get(term)? as f32;
        let total = self.total as f32;
        Some(total / freq)
    }

    pub fn build_vec(&self, inp: &str) -> Option<Vector> {
        let padded = self.get_paddded(inp);
        let n = self.n_for(inp);
        let ng_ids: Vec<_> = Wordgrams::new(&padded, n)
            .filter_map(|i| Some((i, *self.ng_ids.get(i)?)))
            .map(|i| {
                let freq = self.inverted_freq(i.0).unwrap();
                (i.1, freq as f32)
            })
            .collect();

        Some(Vector::create_new_raw(ng_ids))
    }

    #[inline]
    fn n_for(&self, inp: &str) -> usize {
        self.n.min(inp.len())
    }

    #[inline]
    fn get_paddded(&self, inp: &str) -> String {
        let n = self.n_for(inp).saturating_sub(1).max(1);
        ngram_tools::padding(inp, n)
    }
}
