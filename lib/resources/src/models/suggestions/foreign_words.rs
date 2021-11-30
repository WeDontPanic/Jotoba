use std::convert::{TryFrom, TryInto};

/// A foreign suggestion item
#[derive(Clone, Debug)]
pub struct ForeignSuggestion {
    pub text: String,
    pub secondary: Option<String>,
    pub sequence: u32,
    pub occurrences: u32,
    pub hash: eudex::Hash,
}

impl Default for ForeignSuggestion {
    fn default() -> Self {
        Self {
            text: Default::default(),
            secondary: Default::default(),
            sequence: Default::default(),
            occurrences: Default::default(),
            hash: eudex::Hash::new(""),
        }
    }
}

impl From<Vec<u8>> for ForeignSuggestion {
    #[inline]
    fn from(data: Vec<u8>) -> Self {
        ForeignSuggestion::try_from(data.as_slice()).unwrap()
    }
}

impl TryFrom<&[u8]> for ForeignSuggestion {
    type Error = ();

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let sequence_bytes: [u8; 4] = value[0..4].try_into().map_err(|_| ())?;
        let occurrences_bytes: [u8; 4] = value[4..8].try_into().map_err(|_| ())?;
        let hash: [u8; 8] = value[8..16].try_into().map_err(|_| ())?;
        let text = String::from_utf8(value[16..].to_vec()).map_err(|_| ())?;
        Ok(Self {
            text,
            hash: eudex::Hash::from(u64::from_le_bytes(hash)),
            sequence: u32::from_le_bytes(sequence_bytes),
            occurrences: u32::from_le_bytes(occurrences_bytes),
            secondary: None,
        })
    }
}

impl From<ForeignSuggestion> for Vec<u8> {
    #[inline]
    fn from(item: ForeignSuggestion) -> Self {
        let mut out = Vec::with_capacity(16 + item.text.len());

        out.extend(item.sequence.to_le_bytes());
        out.extend(item.occurrences.to_le_bytes());

        let hash: u64 = item.hash.into();
        out.extend(hash.to_le_bytes());

        out.extend(item.text.as_bytes());

        out
    }
}
