use byteorder::{ByteOrder, WriteBytesExt};
use vector_space_model::{document_vector, traits::Encodable, Error};

/// A `document_vector::Document` implementing type for generating new vectors
#[derive(Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct GenDoc {
    terms: Vec<String>,
    seq_ids: Vec<usize>,
}

impl Encodable for GenDoc {
    fn encode<T: ByteOrder>(&self) -> Result<Vec<u8>, Error> {
        let mut encoded = vec![];

        encoded.write_u16::<T>(self.seq_ids.len() as u16)?;

        for seq_id in self.seq_ids.iter() {
            encoded.write_u64::<T>(*seq_id as u64)?;
        }

        Ok(encoded)
    }
}

impl document_vector::Document for GenDoc {
    fn get_terms(&self) -> Vec<String> {
        self.terms.clone()
    }
}

impl GenDoc {
    /// Create a new GenDoc Document
    pub fn new<T: ToString>(terms: Vec<T>, seq_ids: Vec<usize>) -> Self {
        GenDoc {
            terms: terms.into_iter().map(|i| i.to_string()).collect(),
            seq_ids,
        }
    }
}
