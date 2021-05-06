use super::JapaneseExt;

/// Returns a vec of all compounds with the same pitch assigned to the accent (true = pitch up) in
/// the order they appeared in the word text
pub fn calc_pitch(kana_word: &str, drop: i32) -> Option<Vec<(&str, bool)>> {
    let kana_items = split_kana(kana_word).collect::<Vec<_>>();
    let syllable_count = kana_items.len();

    if syllable_count == 0 || drop < 0 || drop > 6 {
        return None;
    }
    let mut kana_items = kana_items.into_iter();

    let first_kana = kana_items.next()?;

    if drop == 0 || drop == 1 {
        if syllable_count == 1 {
            return Some(vec![(first_kana, drop == 1)]);
        } else {
            return Some(vec![
                (first_kana, drop == 1),
                (&kana_word[first_kana.bytes().len()..], drop == 0),
            ]);
        }
    }

    if syllable_count == drop as usize {
        return Some(vec![
            (first_kana, false),
            (&kana_word[first_kana.bytes().len()..], true),
        ]);
    } else {
        let up: usize = kana_items
            .by_ref()
            .take((drop) as usize)
            .map(|i| i.bytes().len())
            .sum();
        return Some(vec![
            (first_kana, false),
            (&kana_word[first_kana.bytes().len()..up], true),
            (&kana_word[up..], true),
        ]);
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
