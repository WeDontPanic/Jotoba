pub mod document;

use crate::metadata::Metadata;
use document::FWordDoc;

// Shortcut for type of index
pub type ForeignIndex = vector_space_model2::Index<FWordDoc, Metadata>;

pub const NATIVE_NGRAM: usize = 3;
pub type NativeIndex = ngindex::NgramIndex<NATIVE_NGRAM, u32>;
