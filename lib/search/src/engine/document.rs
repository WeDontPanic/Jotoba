use std::io::Read;

use bitflags::BitFlag;
use byteorder::{ByteOrder, ReadBytesExt};
use types::jotoba::languages::Language;
use vector_space_model::traits::Decodable;

/// A document belongs to a document-vector and contains the seq_ids of all items who represent
/// this document
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiDocument {
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

/// A document belongs to a document-vector and contains a single seq_id which means this
/// document represents a single resource item.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SingleDocument {
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

/// A sentence document represents a single sentence, referenced by its ID, and a bitmask of
/// supported languages for more efficient searching
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SentenceDocument {
    pub seq_id: u32,
    pub mask: u16,
}

impl Decodable for SentenceDocument {
    #[inline(always)]
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, vector_space_model::Error> {
        let seq_id = data.read_u32::<T>()?;
        let mask = data.read_u16::<T>()?;
        Ok(Self { seq_id, mask })
    }
}

impl SentenceDocument {
    /// Returns true if the given SentenceDocument has a translation for `language`
    #[inline]
    pub fn has_language(&self, language: Language) -> bool {
        let lang_id: i32 = language.into();
        BitFlag::new_with_value(self.mask).get_unchecked(lang_id as u16)
    }
}
