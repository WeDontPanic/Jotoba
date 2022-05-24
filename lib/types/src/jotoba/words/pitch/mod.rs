pub mod raw_data;

#[cfg(feature = "jotoba_intern")]
pub mod border;

use japanese::JapaneseExt;
use serde::{Deserialize, Serialize};

/// Owned pitch entry of a word
#[derive(Clone, Serialize, Deserialize)]
pub struct Pitch {
    pub parts: Vec<PitchPart>,
}

impl Pitch {
    pub fn new(kana: &str, drop: u8) -> Option<Self> {
        let mut kana_items = split_kana(kana).collect::<Vec<_>>();
        kana_items.push("");
        let syllable_count = kana_items.len();

        if syllable_count == 0 || drop > 6 {
            return None;
        }
        let mut kana_items = kana_items.into_iter();

        let first_kana = kana_items.next()?;

        if drop == 0 || drop == 1 {
            if syllable_count == 1 {
                let inner = vec![PitchPart::new(first_kana, drop == 1)];
                return Some(Self::new_raw(inner));
            } else {
                let part1 = PitchPart::new(first_kana, drop == 1);
                let part2 = PitchPart::new(&kana[first_kana.bytes().len()..], drop == 0);
                return Some(Self::new_raw(vec![part1, part2]));
            }
        }

        let up: usize = kana_items
            .by_ref()
            .take((drop - 1) as usize)
            .map(|i| i.bytes().len())
            .sum();

        let parts = vec![
            PitchPart::new(first_kana, false),
            PitchPart::new(
                &kana[first_kana.bytes().len()..first_kana.bytes().len() + up],
                true,
            ),
            PitchPart::new(&kana[first_kana.bytes().len() + up..], false),
        ];

        return Some(Pitch::new_raw(parts));
    }

    #[inline]
    fn new_raw(parts: Vec<PitchPart>) -> Self {
        Self { parts }
    }

    /// Get a reference to the pitch's parts.
    #[inline]
    pub fn parts(&self) -> &[PitchPart] {
        self.parts.as_ref()
    }

    /// Render helper for the template
    #[cfg(feature = "jotoba_intern")]
    pub fn render(&self) -> impl Iterator<Item = (String, &str)> {
        use self::border::{Border, BorderBuilder};
        let mut iter = self.parts.iter().enumerate();

        std::iter::from_fn(move || {
            let (pos, pitch_part) = iter.next()?;

            if pitch_part.part.is_empty() {
                // Don't render under/overline for empty character -- handles the case where the
                // pitch changes from the end of the word to the particle
                return Some((String::new(), ""));
            }

            let h_bord = Border::horizontal(pitch_part.high);
            let mut b_builder = BorderBuilder::new(h_bord);

            if pos != self.parts.len() - 1 {
                b_builder.add(Border::Right);
            }

            let classes = b_builder.build();
            let part_str = pitch_part.part.as_str();
            Some((classes, part_str))
        })
    }
}

/// A single, owned part of a whole pitch entry for a word
#[derive(Clone, Serialize, Deserialize)]
pub struct PitchPart {
    pub part: String,
    pub high: bool,
}

impl PitchPart {
    #[inline]
    pub fn new<S: ToString>(part: S, high: bool) -> Self {
        Self {
            part: part.to_string(),
            high,
        }
    }
}

/// Returns an iterator over all kana characters. The reason for Item to be &str is that 'きゅう'
/// gets split up into ["きゅ", "う"] which can't be represented with only one char
pub fn split_kana(inp: &str) -> impl Iterator<Item = &str> {
    let mut char_indices = inp.char_indices().peekable();

    std::iter::from_fn(move || {
        let (start_idx, _) = char_indices.next()?;
        while let Some(&(next_idx, chr)) = char_indices.peek() {
            if !chr.is_small_kana() {
                return Some(&inp[start_idx..next_idx]);
            }
            char_indices.next();
        }

        Some(&inp[start_idx..])
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_kana() {
        assert!(!"よ".is_small_kana());

        assert!("ょ".is_small_kana());
        assert!("ゃ".is_small_kana());
        assert!("ゅ".is_small_kana());

        assert!("ョ".is_small_kana());
        assert!("ャ".is_small_kana());
        assert!("ュ".is_small_kana());
    }

    #[test]
    fn test_split_kana_small() {
        let inp = "きょうかしょ";
        let out = split_kana(inp).collect::<Vec<_>>();
        assert_eq!(out, vec!["きょ", "う", "か", "しょ"]);
    }

    #[test]
    fn test_split_kana() {
        let inp = "これがすき";
        let out = split_kana(inp).collect::<Vec<_>>();
        assert_eq!(out, vec!["こ", "れ", "が", "す", "き"]);
    }

    #[test]
    fn test_split_kana2() {
        let inp = "";
        let out = split_kana(inp).collect::<Vec<_>>();
        let empty: Vec<&str> = Vec::new();
        assert_eq!(out, empty);
    }
}
