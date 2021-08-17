use std::convert::{TryFrom, TryInto};

/// A foreign suggestion item
#[derive(Clone, Debug)]
pub struct NativeSuggestion {
    pub text: String,
    pub sequence: u32,
}

impl TryFrom<&[u8]> for NativeSuggestion {
    type Error = ();

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let sequence_bytes: [u8; 4] = value[0..4].try_into().map_err(|_| ())?;
        let text = String::from_utf8(value[16..].to_vec()).map_err(|_| ())?;
        Ok(Self {
            text,
            sequence: u32::from_le_bytes(sequence_bytes),
        })
    }
}

impl From<NativeSuggestion> for Vec<u8> {
    #[inline]
    fn from(item: NativeSuggestion) -> Self {
        let mut out = Vec::with_capacity(16 + item.text.len());
        out.extend(item.sequence.to_le_bytes());
        out.extend(item.text.as_bytes());
        out
    }
}
