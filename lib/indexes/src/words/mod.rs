pub mod document;

use crate::metadata::Metadata;
use document::FWordDoc;
use vector_space_model2::DefaultMetadata;

// Shortcut for type of index
pub type ForeignIndex = vector_space_model2::Index<FWordDoc, Metadata>;
pub type NativeIndex = vector_space_model2::Index<u32, DefaultMetadata>;
