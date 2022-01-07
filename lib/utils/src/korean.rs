/// Returns true if `c` is a hangul character
#[inline]
pub fn is_hangul(c: char) -> bool {
    (c >= '\u{AC00}' && c <= '\u{D7AF}')
        || (c >= '\u{1100}' && c <= '\u{11FF}')
        || (c >= '\u{3130}' && c <= '\u{321E}')
}

/// Returns true if `c` is a hangul character
#[inline]
pub fn is_hangul_str(s: &str) -> bool {
    !s.chars().any(|i| !is_hangul(i))
}
