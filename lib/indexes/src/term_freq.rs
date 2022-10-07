use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Index to index frequencies of terms
#[derive(Serialize, Deserialize)]
pub struct TermFreqIndex {
    pub(crate) freqs: HashMap<u32, u32>,
    pub(crate) t_ids: HashMap<String, u32>,
    pub(crate) total: usize,
}

impl TermFreqIndex {
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

    /// Insert a new term into the index or increases the
    /// frequency value of an existing term
    pub fn insert(&mut self, term: String) {
        self.total += 1;

        let freq = self.t_ids.get(&term).and_then(|id| self.freqs.get_mut(&id));
        if let Some(freq) = freq {
            *freq += 1;
            return;
        }

        let new_id = self.t_ids.len() as u32;
        self.t_ids.insert(term, new_id);
        self.freqs.insert(new_id, 1);
    }

    // Remove all terms with frequency `threshold` and treat out of dict
    // ngrams as frequency = `1` to reduce memory usage.
    pub fn compress(&mut self, threshold: usize) {
        self.t_ids.retain(|_, id| {
            let freq = *self.freqs.get(id).unwrap();
            if freq < threshold as u32 {
                self.freqs.remove(id).unwrap();
                return false;
            }
            true
        });
    }

    #[inline]
    pub fn get_id(&self, term: &str) -> Option<u32> {
        self.t_ids.get(term).copied()
    }

    #[inline]
    pub fn freq(&self, term: &str) -> Option<u32> {
        let id = self.get_id(term)?;
        self.freq_by_id(id)
    }

    #[inline]
    pub fn freq_by_id(&self, id: u32) -> Option<u32> {
        self.freqs.get(&id).copied()
    }

    /// Inverted frequency. Out-of-vocab terms return `None`
    #[inline]
    pub fn inv_freq(&self, term: &str) -> Option<f32> {
        let freq = self.freq(term)? as f32;
        let total = self.total as f32;
        Some((total / freq).log2())
    }

    /// Inverted frequency but out-of-vocab terms are treated as freq=1
    #[inline]
    pub fn inv_freq_oov(&self, term: &str) -> f32 {
        let freq = self.freq(term).unwrap_or(1) as f32;
        let total = self.total as f32;
        (total / freq).log2()
    }
}
