use std::collections::HashMap;

use ngram_tools::iter::wordgrams::Wordgrams;
use serde::{Deserialize, Serialize};
use vsm::Vector;

#[derive(Serialize, Deserialize)]
pub struct TermIndex {
    freqs: HashMap<String, u32>,
    t_ids: HashMap<String, u32>,
    total: usize,
}

impl TermIndex {
    pub fn new() -> Self {
        Self {
            freqs: HashMap::new(),
            t_ids: HashMap::new(),
            total: 0,
        }
    }

    /// Returns the amount of indexed terms
    #[inline]
    pub fn len(&self) -> usize {
        self.freqs.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn index_ng(&mut self, gloss: &str, n: usize) {
        let padded = self.get_paddded(gloss, n);
        let n = Self::n_for(gloss, n);
        let ngrams = Wordgrams::new(&padded, n);

        for ngram in ngrams {
            self.insert(ngram.to_string());
        }
    }

    pub fn insert(&mut self, term: String) {
        self.total += 1;
        if let Some(freq) = self.freqs.get_mut(&term) {
            *freq += 1;
            return;
        }

        let new_id = self.t_ids.len() as u32;
        self.t_ids.insert(term.clone(), new_id);
        self.freqs.insert(term.clone(), 1);
    }

    pub fn compress(&mut self, threshold: usize) {
        // Remove all terms with frequency `threshold` and treat out of dict
        // ngrams as 1 to save memory
        self.freqs.retain(|k, v| {
            if *v < threshold as u32 {
                self.t_ids.remove(k).unwrap();
                return false;
            }

            true
        });
    }

    #[inline]
    pub fn freq(&self, term: &str) -> f32 {
        self.freqs.get(term).map(|i| *i as f32).unwrap_or(1.0)
    }

    #[inline]
    pub fn inverted_freq(&self, term: &str) -> f32 {
        let freq = self.freq(term);
        let total = self.total as f32;
        (total / freq).log2()
    }

    #[inline]
    pub fn sim_ng<A, B>(&self, a: A, b: B, n: usize) -> f32
    where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let a_vec = self.build_ng_vec(a, n);
        let b_vec = self.build_ng_vec(b, n);

        let (a_vec, b_vec) = match (a_vec, b_vec) {
            (Some(a), Some(b)) => (a, b),
            _ => return 0.0,
        };

        vec_sim(&a_vec, &b_vec)
    }

    #[inline]
    pub fn build_vec2<A: AsRef<str>>(&self, inp: A) -> Option<Vector> {
        let id = self.t_ids.get(inp.as_ref()).copied().unwrap_or(0);
        let freq = self.freq(inp.as_ref());
        Some(Vector::create_new_raw(vec![(id, freq)]))
    }
    pub fn build_ng_vec<A: AsRef<str>>(&self, inp: A, ng: usize) -> Option<Vector> {
        let inp = inp.as_ref();
        let padded = self.get_paddded(inp, ng);
        let n = Self::n_for(inp, ng);

        let ng_ids: Vec<_> = Wordgrams::new(&padded, n)
            .map(|i| {
                let id = self.t_ids.get(i).copied().unwrap_or(0);
                let freq = self.inverted_freq(i);
                (id, freq as f32)
            })
            .collect();

        Some(Vector::create_new_raw(ng_ids))
    }

    #[inline]
    fn n_for(inp: &str, n: usize) -> usize {
        n.min(inp.len())
    }

    #[inline]
    fn get_paddded(&self, inp: &str, n: usize) -> String {
        let n = Self::n_for(inp, n);
        ngram_tools::padding(inp, n - 1)
    }
}

#[inline]
pub fn vec_sim(a: &Vector, b: &Vector) -> f32 {
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
