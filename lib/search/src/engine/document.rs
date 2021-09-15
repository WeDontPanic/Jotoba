use std::io::Read;

use byteorder::{ByteOrder, ReadBytesExt};
use vector_space_model::traits::Decodable;

/// A document belongs to a document-vector and contains the seq_ids of all words who represent
/// this document
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MultiDocument {
    pub seq_ids: Vec<u32>,
}

impl Decodable for MultiDocument {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, vector_space_model::Error> {
        let seq_id_count = data.read_u16::<T>()?;

        let seq_ids = (0..seq_id_count)
            .map(|_| data.read_u32::<T>())
            .collect::<Result<_, _>>()?;

        Ok(Self { seq_ids })
    }
}

/// A document belongs to a document-vector and contains a seq_id
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct SingleDocument {
    pub seq_id: u32,
}

impl Decodable for SingleDocument {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, vector_space_model::Error> {
        Ok(Self {
            seq_id: data.read_u32::<T>()?,
        })
    }
}
