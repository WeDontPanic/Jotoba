pub mod generate;
pub mod parse;
mod tests;

use super::JapaneseExt;
use itertools::Itertools;

/// Represents a single sentence part which either consisting of kana only or kanji and a kana reading
/// assigned
#[derive(Clone, Debug)]
pub struct SentencePart {
    pub kana: String,
    pub kanji: Option<String>,
}

/// Same as [`SentencePart`] but with referenced substrings instead of an owned String
#[derive(Clone, Copy, Debug)]
pub struct SentencePartRef<'a> {
    pub kana: &'a str,
    pub kanji: Option<&'a str>,
}

impl SentencePart {
    /// Create a new `SentencePart` with kana only
    pub fn new(kana: String) -> Self {
        Self { kana, kanji: None }
    }

    /// Create a new `SentencePart` with kanji value
    pub fn with_kanji(kana: String, kanji: String) -> Self {
        Self {
            kana,
            kanji: Some(kanji),
        }
    }

    /// Encodes a SentencePartRef to string
    #[inline]
    pub fn encode(&self) -> String {
        self.as_ref().encode()
    }

    /// Returns `true` if SentencePart has kanji reading
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.as_ref().has_kanji()
    }

    /// Returns `true` if SentencePart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    #[inline]
    pub fn as_ref(&self) -> SentencePartRef {
        SentencePartRef {
            kana: &self.kana,
            kanji: self.kanji.as_deref(),
        }
    }
}

impl<'a> SentencePartRef<'a> {
    /// Creates a new SentencePartRef
    pub fn new(kana: &'a str) -> Self {
        Self { kana, kanji: None }
    }

    /// Creates a new SentencePartRef with a value for kanji
    pub fn with_kanji(kana: &'a str, kanji: &'a str) -> Self {
        Self {
            kana,
            kanji: Some(kanji),
        }
    }

    /// Encodes a SentencePartRef to a furigana string
    pub fn encode(&self) -> String {
        if let Some(kanji) = self.kanji {
            generate::furigana_block(kanji, self.kana)
        } else {
            self.kana.to_string()
        }
    }

    /// Returns `true` if SentencePart has kanji reading
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.kanji.is_some()
    }

    /// Returns `true` if SentencePart is empty. Since every part has at least to hold kana data
    /// `empty` is already the case if the kana reading is empmty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.kana.trim().is_empty()
    }
}

/// Check wether the passed furigana pairs are representing the given kana text or not
pub fn check_pairs(pars: &[SentencePart], kana: &str) -> bool {
    let s: String = pars.iter().map(|i| i.kana.clone()).collect();

    romaji::RomajiExt::to_hiragana(s.as_str()).replace("・", "")
        == romaji::RomajiExt::to_hiragana(kana).replace("・", "")
}

/// Generates all kanji readins from a kanji and kana string an returns them (kanji, kana)
fn map_readings(kanji: &str, kana: &str) -> Option<Vec<(String, String)>> {
    let kana = kana.chars().filter(|s| !s.is_symbol()).collect::<Vec<_>>();
    let mut kana_pos = strip_until_kanji(kanji.chars());
    let mut kanji_iter = kanji.chars().filter(|i| !i.is_symbol()).skip(kana_pos);

    let mut result: Vec<(String, String)> = Vec::new();

    let mut curr_kanji = Vec::new();
    loop {
        if kana_pos >= kana.len() {
            break;
        }

        // Kana from current position to end
        let curr_kana = &kana[kana_pos..];

        let kk = kanji_iter.clone().collect_vec();

        // Get all chars until next kanji
        let (part_kana, part_kanji) = to_next_kanji(&mut kanji_iter);

        // If last part is kanji only take rest of kana reading
        if part_kana.is_empty() {
            result.push((part_kanji.iter().collect(), curr_kana.iter().collect()));
            break;
        }

        // Current kanji buff
        curr_kanji.clear();
        let mut counter = 1;
        let found = loop {
            if kana_pos >= kana.len() {
                break false;
            }
            curr_kanji.push(kana[kana_pos]);
            kana_pos += 1;

            // Require at least as much kana characters as kanji characters
            if counter < part_kanji.len() {
                counter += 1;
                continue;
            }

            if starts_with(
                curr_kana,
                &curr_kanji,
                &part_kana,
                !has_kanji_after(&kk, part_kanji.len() + part_kana.len()),
            ) {
                break true;
            }

            if curr_kanji.len() >= curr_kana.len() || kana_pos >= kana.len() {
                break false;
            }
            counter += 1;
        };

        if !found {
            // Error
            return None;
        }

        result.push((
            char_arr_to_string(&part_kanji),
            char_arr_to_string(&curr_kanji),
        ));

        for _ in 0..(part_kana.len() + part_kanji.len()) {
            kanji_iter.next();
        }

        kana_pos += part_kana.len();
    }

    Some(result)
}

/// Returns true if there are kanji elements within arr after the given offset
fn has_kanji_after<T>(arr: &[T], offset: usize) -> bool
where
    T: JapaneseExt,
{
    if offset >= arr.len() {
        return false;
    }

    arr[offset..]
        .iter()
        .any(|i| i.is_kanji() || i.is_roman_letter())
}

/// Checks whether 'arr' starts with a*b or not
fn starts_with<T>(arr: &[T], a: &[T], b: &[T], last: bool) -> bool
where
    T: PartialEq + JapaneseExt,
{
    if last {
        if a.len() + b.len() != arr.len() {
            return false;
        }
    } else if a.len() + b.len() > arr.len() {
        return false;
    }

    for (pos, item) in a.iter().enumerate() {
        if arr[pos].to_hiragana() != *item.to_hiragana() {
            return false;
        }
    }

    for (pos, item) in b.iter().enumerate() {
        if arr[pos + a.len()].to_hiragana() != *item.to_hiragana() {
            return false;
        }
    }

    true
}

/// Helper method to collect all items in a
/// Vec<char> into a newly allocated String
#[inline]
fn char_arr_to_string(vec: &[char]) -> String {
    vec.iter().collect()
}

/// Returns all Kanji and kana elements until a new kanji(compound) is reached
fn to_next_kanji<T>(kanji_iter: &mut T) -> (Vec<char>, Vec<char>)
where
    T: Iterator<Item = char> + Clone,
{
    let mut kanji_iter = kanji_iter.clone();
    let kanji = kanji_iter
        .take_while_ref(|i| i.is_kanji() || i.is_symbol() || i.is_roman_letter())
        .collect::<Vec<_>>();
    let kana = kanji_iter
        .take_while_ref(|i| i.is_kana())
        .collect::<Vec<_>>();
    (kana, kanji)
}

/// Truncates everything from a kanji_iterator until a kanji element has reached and returns the
/// amount of trimmed characters
fn strip_until_kanji<T>(mut kanji_iter: T) -> usize
where
    T: Iterator<Item = char>,
{
    let mut i = 0;
    loop {
        if kanji_iter
            .next()
            .map(|i| i.is_kanji() || i.is_symbol() || i.is_roman_letter())
            .unwrap_or(true)
        {
            break i;
        }

        i += 1;
    }
}
