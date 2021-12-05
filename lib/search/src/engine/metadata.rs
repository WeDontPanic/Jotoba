use std::{convert::TryFrom, io::Read};

use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use types::jotoba::languages::Language;
use vector_space_model::{
    metadata::IndexVersion,
    traits::{Decodable, Encodable},
    Error,
};

/// Various metadata for the given Index
#[derive(Debug)]
pub struct Metadata {
    pub version: IndexVersion,
    pub document_count: usize,
    pub language: Language,
}

impl Metadata {
    /// Creates a new `Metadata` with the given values
    #[inline]
    pub fn new(version: IndexVersion, document_count: usize, language: Language) -> Self {
        Self {
            version,
            document_count,
            language,
        }
    }
}

impl vector_space_model::metadata::Metadata for Metadata {
    #[inline]
    fn get_version(&self) -> IndexVersion {
        self.version
    }

    #[inline]
    fn get_document_count(&self) -> usize {
        self.document_count
    }
}

impl Encodable for Metadata {
    fn encode<T: ByteOrder>(&self) -> Result<Vec<u8>, Error> {
        let mut out = vec![];

        out.write_u8(self.version as u8)?;
        out.write_u64::<T>(self.document_count as u64)?;
        out.write_i32::<T>(self.language.into())?;

        Ok(out)
    }
}

impl Decodable for Metadata {
    fn decode<T: ByteOrder, R: Read>(mut data: R) -> Result<Self, Error> {
        let version = IndexVersion::try_from(data.read_u8()?)?;
        let document_count = data.read_u64::<T>()? as usize;
        let lang = data.read_i32::<T>()?;

        let language = Language::try_from(lang).map_err(|_| Error::Decode)?;

        Ok(Self {
            version,
            document_count,
            language,
        })
    }
}
