use jp_utils::JapaneseExt;

const KANA_BEGIN: u32 = 12353;
const MAX_COMPR_VAL: u32 = 190;

/// Compresses kana input to reduce memory size
#[inline]
pub fn compress_kana(inp: &str) -> Option<String> {
    if !inp.is_kana() {
        return None;
    }

    inp.chars()
        .map(|i| char::from_u32(i as u32 - KANA_BEGIN))
        .collect::<Option<String>>()
}

/// Decompresses a compressed String back to kana. If the input
/// contains a kana character, the input gets returned
#[inline]
pub fn decompress_kana(inp: &str) -> Option<String> {
    // if even one kana character is available,
    // the input can't be compressed
    if inp.has_kana() {
        return Some(inp.to_string());
    }

    inp.chars()
        .map(|i| {
            let int = i as u32;
            (int <= MAX_COMPR_VAL)
                .then(|| char::from_u32(int + KANA_BEGIN))
                .flatten()
        })
        .collect::<Option<String>>()
}
