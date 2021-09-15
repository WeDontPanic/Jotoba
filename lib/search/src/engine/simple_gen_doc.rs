use vector_space_model::document_vector;

/// A `document_vector::Document` implementing type for generating new vectors
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct GenDoc {
    terms: Vec<String>,
}

impl document_vector::Document for GenDoc {
    #[inline]
    fn get_terms(&self) -> Vec<String> {
        self.terms.clone()
    }
}

impl GenDoc {
    /// Create a new GenDoc Document
    #[inline]
    pub fn new<T: ToString>(terms: Vec<T>) -> Self {
        GenDoc {
            terms: terms.into_iter().map(|i| i.to_string()).collect(),
        }
    }
}
