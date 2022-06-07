use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt};
use vector_space_model2::traits::{Decodable, Encodable};

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct FWordDoc {
    pub items: Vec<FWordDocItem>,
}

impl FWordDoc {
    #[inline]
    pub fn new(items: Vec<FWordDocItem>) -> Self {
        Self { items }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct FWordDocItem {
    pub seq_id: u32,
    pub positions: Vec<u16>,
}

impl Encodable for FWordDocItem {
    fn encode<T: byteorder::ByteOrder>(&self) -> Result<Vec<u8>, vector_space_model2::Error> {
        let mut out = Vec::with_capacity(8);
        out.write_u32::<T>(self.seq_id)?;
        out.write_u8(self.positions.len() as u8)?;

        for pos in &self.positions {
            out.write_u16::<T>(*pos)?;
        }

        Ok(out)
    }
}

impl FWordDocItem {
    #[inline]
    pub fn new(seq_id: u32, positions: Vec<u16>) -> Self {
        Self { seq_id, positions }
    }
}

impl Encodable for FWordDoc {
    fn encode<T: byteorder::ByteOrder>(&self) -> Result<Vec<u8>, vector_space_model2::Error> {
        let mut out = vec![];
        out.write_u8(self.items.len() as u8)?;

        for item in &self.items {
            out.extend(item.encode::<T>()?);
        }

        Ok(out)
    }
}

impl Decodable for FWordDoc {
    #[inline]
    fn decode<T: byteorder::ByteOrder, R: Read>(
        mut data: R,
    ) -> Result<Self, vector_space_model2::Error> {
        let len = data.read_u8()? as usize;

        let mut items = Vec::with_capacity(len);

        for _ in 0..len {
            let seq_id = data.read_u32::<T>()?;
            let pos_len = data.read_u8()?;

            let mut pos = Vec::with_capacity(pos_len as usize);

            for _ in 0..pos_len {
                pos.push(data.read_u16::<T>()?);
            }

            items.push(FWordDocItem::new(seq_id, pos));
        }

        Ok(FWordDoc { items })
    }
}
