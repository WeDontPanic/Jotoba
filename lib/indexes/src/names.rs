pub const FOREIGN_NGRAM: usize = 3;
pub type ForeignIndex = ngindex::NgramIndex<FOREIGN_NGRAM, u32>;

pub const NATIVE_NGRAM: usize = 3;
pub type NativeIndex = ngindex::NgramIndex<NATIVE_NGRAM, u32>;
