/// Returns the real amount of characters in a string
pub fn real_string_len(s: &str) -> usize {
    // We should probably use grapheme clusters here
    s.chars().count()
}

pub fn char_eq_str(c: char, s: &str) -> bool {
    let mut chars = s.chars();
    let is = chars.next().map(|i| i == c).unwrap_or_default();
    is && chars.next().is_none()
}
