use std::io::Read;

use bitflags::BitFlag;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use types::jotoba::languages::Language;
use vector_space_model2::traits::{Decodable, Encodable};

/// A sentence document represents a single sentence, referenced by its ID, and a bitmask of
/// supported languages for more efficient searching
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SentenceDocument {
    pub seq_id: u32,
    pub mask: u16,
}

impl SentenceDocument {
    /// Returns true if the given SentenceDocument has a translation for `language`
    #[inline]
    pub fn has_language(&self, language: Language) -> bool {
        let lang_id: i32 = language.into();
        BitFlag::new_with_value(self.mask).get_unchecked(lang_id as u16)
    }
}

impl Decodable for SentenceDocument {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, vector_space_model2::Error> {
        let seq_id = data.read_u32::<T>()?;
        let mask = data.read_u16::<T>()?;
        Ok(Self { seq_id, mask })
    }
}

impl Encodable for SentenceDocument {
    #[inline]
    fn encode<T: ByteOrder>(&self) -> Result<Vec<u8>, vector_space_model2::Error> {
        let mut encoded = Vec::with_capacity(6);
        encoded.write_u32::<T>(self.seq_id)?;
        encoded.write_u16::<T>(self.mask)?;
        Ok(encoded)
    }
}
