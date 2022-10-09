use ngram_tools::iter::wordgrams::Wordgrams;
use serde::{Deserialize, Serialize};
use vsm::Vector;

use crate::term_freq::{TermFreqIndex, VecBuilder};

/// Wrapper around Term frequency index counting ngrams of terms instead of the terms intelf.
#[derive(Serialize, Deserialize)]
pub struct NgFreqIndex {
    n: usize,
    index: TermFreqIndex,
}

impl NgFreqIndex {
    pub fn new(n: usize) -> Self {
        let index = TermFreqIndex::new();
        Self { n, index }
    }

    /// Returns the amount of indexed terms
    #[inline]
    pub fn len(&self) -> usize {
        self.index.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn compress(&mut self, threshold: usize) {
        self.index.compress(threshold)
    }

    pub fn insert(&mut self, gloss: &str) {
        let padded = self.get_padded(gloss);
        let n = Self::n_for(gloss, self.n);
        let ngrams = Wordgrams::new(&padded, n);

        for ngram in ngrams {
            self.index.insert(ngram.to_string());
        }
    }

    #[inline]
    pub fn build_vec_cntx<A: AsRef<str>>(&self, builder: &mut VecBuilder, inp: A) -> Vector {
        self.build_custom_vec_cntx(builder, inp, |freq, tot| (tot / freq).log2())
    }

    #[inline]
    pub fn build_vec<A: AsRef<str>>(&self, inp: A) -> Vector {
        self.build_custom_vec(inp, |freq, tot| (tot / freq).log2())
    }

    pub fn build_custom_vec<A, F>(&self, inp: A, inv_freq: F) -> Vector
    where
        A: AsRef<str>,
        F: Fn(f32, f32) -> f32,
    {
        let inp = inp.as_ref();
        let padded = self.get_padded(inp);
        let n = Self::n_for(inp, self.n);

        let mut no_hit_counter = 0;
        let ng_ids: Vec<_> = Wordgrams::new(&padded, n)
            .map(|i| {
                let id = self.index.t_ids.get(i).copied().unwrap_or_else(|| {
                    no_hit_counter += 1;
                    self.index.total as u32 + no_hit_counter
                });

                //let freq = self.index.inv_freq_oov(i);
                let t_freq = self.index.freq_by_id(id).unwrap_or(1) as f32;
                let weight = (inv_freq)(t_freq, self.index.total as f32);
                (id, weight)
            })
            .collect();

        Vector::create_new_raw(ng_ids)
    }

    pub fn build_custom_vec_cntx<A, F>(
        &self,
        builder: &mut VecBuilder,
        inp: A,
        inv_freq: F,
    ) -> Vector
    where
        A: AsRef<str>,
        F: Fn(f32, f32) -> f32,
    {
        let inp = inp.as_ref();
        let padded = self.get_padded(inp);
        let n = Self::n_for(inp, self.n);

        let ng_ids: Vec<_> = Wordgrams::new(&padded, n)
            .map(|i| {
                let id = builder.get_or_insert_id(i);

                let t_freq = self.index.freq_by_id(id).unwrap_or(1) as f32;
                let weight = (inv_freq)(t_freq, self.index.total as f32);

                (id, weight)
            })
            .collect();

        Vector::create_new_raw(ng_ids)
    }

    #[inline]
    fn n_for(inp: &str, n: usize) -> usize {
        n.min(inp.len())
    }

    #[inline]
    fn get_padded(&self, inp: &str) -> String {
        let n = Self::n_for(inp, self.n);
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

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("musik", 1)]
    #[test_case("musik", 2)]
    #[test_case("musik", 3)]
    #[test_case("ki", 1)]
    #[test_case("ki", 2)]
    #[test_case("ki", 3)]
    fn test_single(term: &str, n: usize) {
        let mut ngindex = NgFreqIndex::new(n);
        ngindex.insert(term);

        let music_vec = ngindex.build_vec(term);
        let term_len = term.len();

        // Check proper length of vectors
        let pad_len = n.saturating_sub(1);
        let tot_len = pad_len * 2 + term_len;
        if term_len < n {
            assert_eq!(music_vec.sparse().len(), tot_len - n);
        } else {
            assert_eq!(music_vec.sparse().len(), tot_len - n + 1);
        }
    }

    #[test]
    fn test_freq() {
        let mut ngindex = NgFreqIndex::new(2);
        ngindex.insert("huhu");

        let freq = ngindex.index.freq("hu");
        assert_eq!(freq, Some(2));
    }

    #[test]
    fn test_sim() {
        let mut ngindex = NgFreqIndex::new(3);
        ngindex.insert("freund");
        ngindex.insert("hund");
        ngindex.insert("kunde");
        ngindex.insert("bund");

        let kund = ngindex.build_vec("kund");

        let kunde = ngindex.build_vec("kunde");
        let hund = ngindex.build_vec("hund");

        let sim_kund_kunde = vec_sim(&kund, &kunde);
        let sim_kund_hund = vec_sim(&kund, &hund);

        assert!(sim_kund_kunde > sim_kund_hund);
    }
}
