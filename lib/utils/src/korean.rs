/// Returns true if `c` is a hangul character
#[inline]
pub fn is_hangul(c: char) -> bool {
    (c >= '\u{AC00}' && c <= '\u{D7AF}')
        || (c >= '\u{1100}' && c <= '\u{11FF}')
        || (c >= '\u{3130}' && c <= '\u{321E}')
}

sabi::sabi! {
    /// Returns true if `c` is a hangul character
    #[inline]
    公開 関数 is_hangul_str(ハングルの文字列: &str) -> bool{
        !ハングルの文字列.chars().any(|i| !is_hangul(i))
    }
}
