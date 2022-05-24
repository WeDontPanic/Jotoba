use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Store for pitch values. There are max 4 pitch values with each 3 bits. This
/// is why we store it efficiently in a u16
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct PitchValues {
    raw: u16,
}

impl PitchValues {
    pub fn new(values: &[u8]) -> Self {
        assert!(values.len() <= 4);

        let mut raw: u16 = 0;

        for (pos, val) in values.iter().enumerate() {
            assert!(*val <= 6);
            let shift = pos as u16 * 3;
            raw |= (*val as u16) << shift;
        }

        raw |= (values.len() as u16) << 12;

        Self { raw }
    }

    #[inline]
    pub fn count(&self) -> u8 {
        (self.raw >> 12) as u8
    }

    #[inline]
    pub fn get(&self, pos: u8) -> Option<u8> {
        (pos < self.count()).then(|| (self.raw >> (pos as u16 * 3)) as u8 & 0b00000111)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        (0..self.count()).map(|i| self.get(i).unwrap())
    }
}

impl Debug for PitchValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (pos, p) in self.iter().enumerate() {
            if pos > 0 {
                write!(f, "|")?;
            }
            write!(f, "{p}")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pitch_value() {
        assert_eq!(PitchValues::new(&[0]).count(), 1);
        assert_eq!(PitchValues::new(&[0]).get(0), Some(0));
        assert_eq!(PitchValues::new(&[0]).get(1), None);

        assert_eq!(PitchValues::new(&[6, 6]).count(), 2);
        assert_eq!(PitchValues::new(&[6, 6]).get(0), Some(6));
        assert_eq!(PitchValues::new(&[6, 6]).get(1), Some(6));

        assert_eq!(PitchValues::new(&[1, 6, 0]).count(), 3);
        assert_eq!(PitchValues::new(&[1, 6, 0]).get(2), Some(0));
        assert_eq!(PitchValues::new(&[]).count(), 0);
    }
}
