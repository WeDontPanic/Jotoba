pub mod foreign;

// Shortcut for type of index
//pub type ForeignIndex = VSMIndexSimple<u32>;
pub type ForeignIndex = foreign::ForeignIndex;

pub const NATIVE_NGRAM: usize = 3;
pub type NativeIndex = ngindex::NgramIndex<NATIVE_NGRAM, u32>;
