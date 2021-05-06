use itertools::Itertools;
use std::iter::Peekable;

use super::{CharType, JapaneseExt};

/// Create SentenceParts out of an input sencence which has to be passed
/// once written in kanji and once in kana characters
pub fn furigana_pairs(kanji: &str, kana: &str) -> Option<Vec<SentencePart>> {
    if !kana.is_kana() || !kanji.is_japanese() || kanji.is_empty() || kana.is_empty() {
        return None;
    }
    let kana = kana.replace("・", "").replace("、", "");
    let kana = kana.as_str();
    let kanji = kanji.replace("・", "").replace("、", "");
    let kanji = kanji.as_str();

    //let mut kanji_readings = kanji_readings(kanji, kana).into_iter();
    let kanji_readings = furi_algo(kanji, kana);
    if kanji_readings.is_none() || !kanji.has_kana() {
        return Some(vec![SentencePart {
            kanji: Some(kanji.to_owned()),
            kana: kana.to_owned(),
        }]);
    }

    let mut kanji_readings = kanji_readings.unwrap().into_iter().map(|(_, kana)| kana);

    let mut parts: Vec<SentencePart> = Vec::new();
    let mut last_char_type: Option<CharType> = None;

    let mut word_buf = String::new();

    for curr_char in kanji.chars() {
        let curr_char_type = curr_char.get_text_type();

        if last_char_type.is_some() && last_char_type.unwrap() != curr_char_type {
            // If char type changes
            let part = SentencePart {
                kana: {
                    if last_char_type.unwrap() == CharType::Kana {
                        word_buf.clone()
                    } else {
                        kanji_readings.next().unwrap_or_default()
                    }
                },
                kanji: (last_char_type.unwrap() == CharType::Kanji
                    || last_char_type.unwrap() == CharType::Other)
                    .then(|| word_buf.clone()),
            };
            parts.push(part);
            word_buf.clear();
        }

        word_buf.push(curr_char);

        last_char_type = Some(curr_char_type);
    }

    parts.push(SentencePart {
        kana: {
            if last_char_type.unwrap() == CharType::Kana {
                word_buf.clone()
            } else {
                kanji_readings.next().unwrap_or_default()
            }
        },
        kanji: (last_char_type.unwrap() == CharType::Kanji
            || last_char_type.unwrap() == CharType::Other)
            .then(|| word_buf.clone()),
    });
    if !furigana_pairs_correct(&&parts, kana) {
        return Some(vec![SentencePart {
            kanji: Some(kanji.to_owned()),
            kana: kana.to_owned(),
        }]);
    }
    Some(parts)
}

/// Generates sentence parts from stringy-furigana
pub fn furigana_from_str(input: &str) -> Vec<SentencePart> {
    let mut in_furi_pairs = false;
    let mut bef_splitter = false;
    let mut result: Vec<SentencePart> = Vec::new();

    let mut curr_part = SentencePart::default();

    for c in input.chars() {
        if c == '[' {
            bef_splitter = true;
            in_furi_pairs = true;
            if !curr_part.is_empty() {
                result.push(curr_part.clone());
                curr_part.clear();
            }
            continue;
        }

        if c == ']' {
            in_furi_pairs = false;
            result.push(curr_part.clone());
            curr_part.clear();
            continue;
        }

        if c == '|' && in_furi_pairs {
            bef_splitter = false;
            continue;
        }

        if in_furi_pairs {
            if bef_splitter {
                if let Some(kanji) = curr_part.kanji.as_mut() {
                    kanji.push(c);
                } else {
                    curr_part.kanji = Some(String::from(c));
                }
            } else {
                curr_part.kana.push(c);
            }
        } else {
            curr_part.kana.push(c);
        }
    }
    result.push(curr_part);

    result
}

/// Check wether the passed furigana pairs are representing the given kana text or not
pub fn furigana_pairs_correct(pars: &Vec<SentencePart>, kana: &str) -> bool {
    let s: String = pars.into_iter().map(|i| i.kana.clone()).collect();

    romaji::RomajiExt::to_hiragana(s.as_str()).replace("・", "")
        == romaji::RomajiExt::to_hiragana(kana).replace("・", "")
}

pub fn format_pairs(pairs: Vec<SentencePart>) -> Vec<SentencePart> {
    pairs
        .into_iter()
        .map(|i| {
            if i.kana.is_empty() && i.kanji.is_some() {
                SentencePart {
                    kana: i.kanji.unwrap(),
                    kanji: None,
                }
            } else {
                i
            }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SentencePart {
    pub kana: String,
    pub kanji: Option<String>,
}

impl SentencePart {
    /// Make the kana reading good looking as furigana text If the kanji count matches with kana
    /// count, a space will be added between each char
    pub fn as_furigana(&self) -> String {
        if let Some(ref kanji) = self.kanji {
            let kana_len = self.kana.chars().count();
            let kanji_len = kanji.chars().count();
            if kana_len == kanji_len {
                self.kana
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(1)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join(" ")
            } else {
                self.kana.clone()
            }
        } else {
            self.kana.clone()
        }
    }

    fn clear(&mut self) {
        self.kana = String::new();
        self.kanji = None;
    }

    fn is_empty(&self) -> bool {
        self.kana.is_empty() && self.kanji.is_none()
    }
}

/// Generates all kanji readins from a kanji and kana string an returns them (kanji, kana)
fn furi_algo(kanji: &str, kana: &str) -> Option<Vec<(String, String)>> {
    let mut kanji_iter = kanji.chars().into_iter().peekable();
    let kana = kana.chars().into_iter().collect::<Vec<_>>();
    let mut kana_pos = strip_until_kanji(&mut kanji_iter);

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
            result.push((
                part_kanji.into_iter().collect(),
                curr_kana.into_iter().collect(),
            ));
            break;
        }

        // Current kanji buff
        curr_kanji.clear();
        let found = loop {
            curr_kanji.push(kana[kana_pos]);
            kana_pos += 1;

            if starts_with(
                &curr_kana,
                &curr_kanji,
                &part_kana,
                !has_kanji_after(&kk, part_kanji.len() + part_kana.len()),
            ) {
                break true;
            }

            if curr_kanji.len() >= curr_kana.len() || kana_pos >= kana.len() {
                break false;
            }
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

/// Returns true if there are kanji
/// elements within arr after the given offset
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
    } else {
        if a.len() + b.len() > arr.len() {
            return false;
        }
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
fn char_arr_to_string(vec: &Vec<char>) -> String {
    vec.iter().collect()
}

/// Returns all Kanji and kana elements until a new kanji(compound) is reached
fn to_next_kanji<T>(kanji_iter: &mut Peekable<T>) -> (Vec<char>, Vec<char>)
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
fn strip_until_kanji<T>(kanji_iter: &mut Peekable<T>) -> usize
where
    T: Iterator<Item = char>,
{
    let mut i = 0;
    loop {
        if kanji_iter
            .peek()
            .map(|i| i.is_kanji() || i.is_symbol() || i.is_roman_letter())
            .unwrap_or(true)
        {
            break i;
        }

        kanji_iter.next();
        i += 1;
    }
}
