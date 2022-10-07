use serde::{Deserialize, Serialize};

use crate::ng_freq::NgFreqIndex;
use std::ops::Deref;

pub const N: usize = 3;
pub type WordVecIndex = ngindex::NgramIndex<N, u32>;

/// Japanese word index
#[derive(Serialize, Deserialize)]
pub struct NativeIndex {
    /// Japanese Word index
    pub index: WordVecIndex,
    /// Ng-Term frequency index
    pub tf_index: NgFreqIndex,
}

impl NativeIndex {
    pub fn new(vsm_index: WordVecIndex, ng_index: NgFreqIndex) -> Self {
        Self {
            index: vsm_index,
            tf_index: ng_index,
        }
    }

    #[inline]
    pub fn index(&self) -> &WordVecIndex {
        &self.index
    }

    #[inline]
    pub fn tf_index(&self) -> &NgFreqIndex {
        &self.tf_index
    }
}

impl Deref for NativeIndex {
    type Target = WordVecIndex;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.index()
    }
}
