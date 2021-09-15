use std::io::Read;

use bitflags::BitFlag;
use byteorder::{ByteOrder, ReadBytesExt};
use resources::parse::jmdict::languages::Language;
use vector_space_model::traits::Decodable;

/// A document belongs to a document-vector and contains a seq_id
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct SentenceDocument {
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
