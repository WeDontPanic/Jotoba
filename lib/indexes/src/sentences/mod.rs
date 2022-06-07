pub mod document;

use crate::metadata::Metadata;
use document::SentenceDocument;
use vector_space_model2::DefaultMetadata;

// Shortcut for type of index
pub type NativeIndex = vector_space_model2::Index<SentenceDocument, DefaultMetadata>;
pub type ForeignIndex = vector_space_model2::Index<SentenceDocument, Metadata>;
