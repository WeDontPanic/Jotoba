use std::io::Read;

use byteorder::{ByteOrder, ReadBytesExt};
use vector_space_model::traits::Decodable;

/// A retrieved document containing seq_ids and optionally the text of the document for debugging
/// purposes. This is retrieved from a `vector_store`
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Document {
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
