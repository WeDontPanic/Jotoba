use std::io::Read;

use byteorder::{ByteOrder, ReadBytesExt};
use vector_space_model::traits::Decodable;

/// A document belongs to a document-vector and contains the seq_ids of all words who represent
/// this document
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Document {
    pub seq_ids: Vec<usize>,
}

impl Decodable for Document {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, vector_space_model::Error> {
        let seq_id_count = data.read_u16::<T>()?;

        let seq_ids = (0..seq_id_count)
            .map(|_| data.read_u64::<T>().map(|i| i as usize))
            .collect::<Result<_, _>>()?;

        Ok(Self { seq_ids })
    }
}
