use vector_space_model2::DefaultMetadata;

// Index shortcuts
//pub type ForeignIndex = vector_space_model2::Index<Vec<u32>, DefaultMetadata>;
pub type ForeignIndex = ngindex::NGIndex<Vec<u32>>;
pub type NativeIndex = vector_space_model2::Index<Vec<u32>, DefaultMetadata>;
