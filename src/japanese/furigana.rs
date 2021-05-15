use itertools::Itertools;

use super::JapaneseExt;

/// Represents a single sentence part which either consisting of kana only or kanji and a kana reading
/// assigned
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SentencePart {
    pub kana: String,
    pub kanji: Option<String>,
}

/// Same as [`SentencePart`] but with referenced substrings instead of an owned String
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SentencePartRef<'a> {
    pub kana: &'a str,
    pub kanji: Option<&'a str>,
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
}

/// Create SentenceParts out of an input sencence
///
/// Equal to [`furigana_checked`] but doesn't return an Option
pub fn pairs(kanji: &str, kana: &str) -> Vec<SentencePart> {
    pairs_checked(kanji, kana).unwrap_or_else(|| vec![default_pair(kanji, kana)])
}

/// Create SentenceParts out of an input sencence which has to be passed
/// once written in kanji and once in kana characters
pub fn pairs_checked(kanji: &str, kana: &str) -> Option<Vec<SentencePart>> {
    if !kana.is_kana() || !kanji.is_japanese() || kanji.is_empty() || kana.is_empty() {
        return None;
    }
    let kana = kana.replace("・", "").replace("、", "");
    let kanji = kanji.replace("・", "").replace("、", "");

    println!("furis: {}, {}", kanji, kana);
    let mut furis = calc_kanji_readings(&kanji, &kana)?.into_iter();

    let parts = super::text_parts(&kanji)
        .map(|part| {
            if part.has_kanji() {
                Some(SentencePart {
                    kanji: Some(part.to_owned()),
                    kana: furis.next()?.1,
                })
            } else {
                Some(SentencePart {
                    kanji: None,
                    kana: part.to_owned(),
                })
            }
        })
        .collect::<Option<Vec<SentencePart>>>()?;

    println!("{:?}", parts);
    Some(parts)
}

/// Parses a furigana string into corresponding SentencePartRef's
/// Expects the input to be valid and each kanji having its own furigana reading assigned
/// In case not every kanji character has its own kana reading assigned, call [`from_str_compound`]
/// instead
pub fn from_str<'a>(input: &'a str) -> impl Iterator<Item = SentencePartRef<'a>> {
    let mut char_iter = input.char_indices().multipeek();

    let mut kanji_pos: Option<i8> = None;
    std::iter::from_fn(move || {
        let (mut pos, start) = char_iter.next()?;

        if start == '[' {
            kanji_pos = Some(-1);
        }

        if let Some(k_pos) = kanji_pos {
            let (a_pos, start) = if k_pos == -1 {
                kanji_pos = Some(0);
                char_iter.next()?
            } else {
                (pos, start)
            };
            let k_pos = kanji_pos.unwrap();

            if start == '|' {
                kanji_pos = None;
                while let Some(v) = char_iter.next() {
                    if v.1 == ']' {
                        pos = v.0 + v.1.len_utf8();
                        break;
                    }
                }
            } else {
                kanji_pos = Some(k_pos + 1);

                // Find the window of the kana reading for the current kanji
                let mut pipe_counter = 0;
                let kana_window = loop {
                    let peeked = char_iter.peek()?;
                    if peeked.1 != '|' {
                        continue;
                    }

                    pipe_counter += 1;

                    if pipe_counter <= k_pos {
                        continue;
                    }

                    let start = peeked.0;
                    let end = loop {
                        let peeked = char_iter.peek()?;
                        if peeked.1 == '|' || peeked.1 == ']' {
                            break peeked.0;
                        }
                    };
                    break (start + 1, end);
                };
                char_iter.reset_peek();

                return Some(SentencePartRef {
                    kanji: Some(&input[a_pos..a_pos + start.len_utf8()]),
                    kana: &input[kana_window.0..kana_window.1],
                });
            }
        }

        // Kana only
        while let Some(&(p, b)) = char_iter.peek() {
            // Peek up to the next furigana block
            if b == '[' {
                return Some(SentencePartRef {
                    kana: &input[pos..p],
                    kanji: None,
                });
            }
            char_iter.next();
        }

        // String could end with kana
        Some(SentencePartRef {
            kanji: None,
            kana: &input[pos..],
        })
    })
}

/// Same as [`from_str`] but treats kanji as compounds, which means kanji don't need a separate
/// kana reading assigned within the furigana window
pub fn from_str_compound<'a>(furi_string: &'a str) -> impl Iterator<Item = SentencePartRef<'a>> {
    let mut char_iter = furi_string.char_indices().peekable();

    std::iter::from_fn(move || {
        let (pos, start) = char_iter.next()?;

        if start == '[' {
            // Current part is a furigana block

            // Get position of the nex '|' and ']' chars since they should exists here
            let to_splitter = char_iter.find(|i| i.1 == '|').map(|i| i.0)?;
            let to_end = char_iter.find(|i| i.1 == ']').map(|i| i.0)?;

            let kanji = &furi_string[pos + 1..to_splitter];
            let kana = &furi_string[to_splitter + 1..to_end];

            if !kana.is_empty() {
                return Some(SentencePartRef {
                    kanji: Some(kanji),
                    kana,
                });
            } else {
                // Some furigana blocks don't have kana
                return Some(SentencePartRef {
                    kanji: None,
                    kana: kanji,
                });
            }
        } else {
            // Kana only
            while let Some(&(p, b)) = char_iter.peek() {
                // Peek up to the next furigana block
                if b == '[' {
                    return Some(SentencePartRef {
                        kana: &furi_string[pos..p],
                        kanji: None,
                    });
                }
                char_iter.next();
            }
        }

        Some(SentencePartRef {
            kanji: None,
            kana: &furi_string[pos..],
        })
    })
}

/// Check wether the passed furigana pairs are representing the given kana text or not
pub fn check_pairs(pars: &[SentencePart], kana: &str) -> bool {
    let s: String = pars.iter().map(|i| i.kana.clone()).collect();

    romaji::RomajiExt::to_hiragana(s.as_str()).replace("・", "")
        == romaji::RomajiExt::to_hiragana(kana).replace("・", "")
}

/// Returns a default [`SentencePart`], used as alternative if furigana algorithm calculated a
/// wrong result. This happens when the kanji reading is not equal with the kana reading
pub fn default_pair(kanji: &str, kana: &str) -> SentencePart {
    SentencePart {
        kana: kana.to_owned(),
        kanji: (!kanji.is_empty()).then(|| kanji.to_owned()),
    }
}

/// Generates all kanji readins from a kanji and kana string an returns them (kanji, kana)
fn calc_kanji_readings(kanji: &str, kana: &str) -> Option<Vec<(String, String)>> {
    let kana = kana.chars().collect::<Vec<_>>();
    let mut kana_pos = strip_until_kanji(kanji.chars());
    let mut kanji_iter = kanji.chars().skip(kana_pos);

    println!("kana pos: {}, kanji_iter: {:?}", kana_pos, kanji_iter);

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
