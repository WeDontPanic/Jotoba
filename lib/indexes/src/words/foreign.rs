use crate::ng_freq::NgFreqIndex;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use vsm::presets::VSMIndexSimple;

/// N-gram with for string similarity (NgFreqIndex) index
pub type WordVecIndex = VSMIndexSimple<u32>;
pub const NG_FREQ_N: usize = 3;

#[derive(Serialize, Deserialize)]
pub struct ForeignIndex {
    pub vsm_index: WordVecIndex,
    pub ng_index: NgFreqIndex,
}

impl ForeignIndex {
    pub fn new(vsm_index: WordVecIndex, ng_index: NgFreqIndex) -> Self {
        Self {
            vsm_index,
            ng_index,
        }
    }

    #[inline]
    pub fn vsm_index(&self) -> &WordVecIndex {
        &self.vsm_index
    }

    #[inline]
    pub fn ng_index(&self) -> &NgFreqIndex {
        &self.ng_index
    }
}

impl Deref for ForeignIndex {
    type Target = WordVecIndex;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.vsm_index()
    }
}
