use std::convert::{TryFrom, TryInto};

/// A foreign suggestion item
#[derive(Clone, Debug)]
pub struct NativeSuggestion {
    pub text: String,
    pub sequence: u32,
}

impl From<Vec<u8>> for NativeSuggestion {
    #[inline]
    fn from(data: Vec<u8>) -> Self {
        NativeSuggestion::try_from(data.as_slice()).unwrap()
    }
}

impl TryFrom<&[u8]> for NativeSuggestion {
    type Error = ();

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let sequence_bytes: [u8; 4] = value[0..4].try_into().map_err(|_| ())?;
        let text = String::from_utf8(value[4..].to_vec()).map_err(|_| ())?;
        Ok(Self {
            text,
            sequence: u32::from_le_bytes(sequence_bytes),
        })
    }
}

impl From<NativeSuggestion> for Vec<u8> {
    #[inline]
    fn from(item: NativeSuggestion) -> Self {
        let mut out = Vec::with_capacity(4 + item.text.len());
        out.extend(item.sequence.to_le_bytes());
        out.extend(item.text.as_bytes());
        out
    }
}
