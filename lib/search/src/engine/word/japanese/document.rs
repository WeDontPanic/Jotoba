use std::io::Read;

use byteorder::{ByteOrder, ReadBytesExt};
use vector_space_model::traits::Decodable;

/// A document belongs to a document-vector and contains the seq_ids of all words who represent
/// this document
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct Document {
    pub seq_id: u32,
}

impl Decodable for Document {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, vector_space_model::Error> {
        Ok(Self {
            seq_id: data.read_u32::<T>()?,
        })
    }
}
