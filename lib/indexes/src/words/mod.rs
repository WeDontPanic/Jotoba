pub mod document;

use crate::metadata::Metadata;
use document::FWordDoc;
use vsm::presets::VSMIndexSimple;

// Shortcut for type of index
pub type ForeignIndex2 = vector_space_model2::Index<FWordDoc, Metadata>;

pub type ForeignIndex = VSMIndexSimple<u32>;

pub const NATIVE_NGRAM: usize = 3;
pub type NativeIndex = ngindex::NgramIndex<NATIVE_NGRAM, u32>;
