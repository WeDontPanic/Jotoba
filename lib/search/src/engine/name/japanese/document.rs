use std::io::Read;

use byteorder::{ByteOrder, ReadBytesExt};
use vector_space_model::{traits::Decodable, Error};

/// A retrieved document containing seq_ids. This is retrieved from a `vector_store`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub seq_ids: Vec<u32>,
}

impl Decodable for Document {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, Error> {
        let seq_id_count = data.read_u16::<T>()?;

        let seq_ids = (0..seq_id_count)
            .map(|_| data.read_u32::<T>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut buf = Vec::new();
        data.read_to_end(&mut buf)?;

        Ok(Self { seq_ids })
    }
}
