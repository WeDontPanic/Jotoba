pub mod furigana;
pub mod guessing;
pub mod radicals;

pub trait ToKanaExt {
    fn to_hiragana(&self) -> String;
    fn to_katakana(&self) -> String;
}

impl ToKanaExt for char {
    #[inline]
    fn to_hiragana(&self) -> String {
        wana_kana::to_hiragana::to_hiragana(self.to_string().as_ref())
    }

    #[inline]
    fn to_katakana(&self) -> String {
        wana_kana::to_katakana::to_katakana(self.to_string().as_ref())
    }
}

impl ToKanaExt for String {
    #[inline]
    fn to_hiragana(&self) -> String {
        wana_kana::to_hiragana::to_hiragana(self.as_ref())
    }

    #[inline]
    fn to_katakana(&self) -> String {
        wana_kana::to_katakana::to_katakana(self.as_ref())
    }
}

impl ToKanaExt for &str {
    #[inline]
    fn to_hiragana(&self) -> String {
        wana_kana::to_hiragana::to_hiragana(self.as_ref())
    }

    #[inline]
    fn to_katakana(&self) -> String {
        wana_kana::to_katakana::to_katakana(self.as_ref())
    }
}

pub fn to_kk_fmt(inp: &str) -> String {
    let inp = inp.to_lowercase();
    let i = inp.replace("nn", "ン");
    wana_kana::to_katakana::to_katakana(&i)
}

pub fn to_hira_fmt(inp: &str) -> String {
    let inp = inp.to_lowercase();
    let i = inp.replace("nn", "ん");
    wana_kana::to_hiragana::to_hiragana(&i)
}

/// Returns `true` if `romaji` is a prefix of `hira` where romaji is romaji text and `hira` is text written in hiragana
#[inline]
pub fn romaji_prefix(romaji: &str, hira: &str) -> bool {
    wana_kana::to_romaji::to_romaji(hira)
        .to_lowercase()
        .starts_with(&romaji.to_lowercase())
}
