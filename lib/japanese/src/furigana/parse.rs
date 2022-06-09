use super::SentencePartRef;
use itertools::{Itertools, MultiPeek};

use std::str::CharIndices;

/// Parses a furigana string into corresponding SentencePartRef's.
/// Expects the input to be correct.
///
/// Input format: `[拝金主義|はい|きん|しゅ|ぎ]は[問|題|もん|だい]`
///
pub fn from_str(input: &str) -> impl Iterator<Item = SentencePartRef<'_>> {
    let mut char_iter = input.char_indices().multipeek();
    let mut kanji_pos: Option<i8> = None;
    std::iter::from_fn(move || {
        let (mut pos, start) = char_iter.next()?;

        if start == '[' {
            kanji_pos = Some(-1);
        }

        if let Some(k_pos) = kanji_pos {
            let (a_pos, a_start) = if k_pos == -1 {
                kanji_pos = Some(0);
                char_iter.next()?
            } else {
                (pos, start)
            };
            let k_pos = kanji_pos.unwrap();

            if a_start == '|' {
                kanji_pos = None;
                for v in &mut char_iter {
                    if v.1 == ']' {
                        pos = v.0 + v.1.len_utf8();
                        break;
                    }
                }
            } else {
                kanji_pos = Some(k_pos + 1);

                let kanji_count = kanji_count(&mut char_iter);
                let furi_count = furi_count(start, &mut char_iter);

                // In case the amount of kanji and readings isn't equal, assign the kanji compound
                // the first reading
                if k_pos == 0 && kanji_count != furi_count {
                    let to_splitter = char_iter.find(|i| i.1 == '|').map(|i| i.0)?;
                    let to_end = char_iter.find(|i| i.1 == ']').map(|i| i.0)?;

                    let kanji = &input[pos + 1..to_splitter];
                    let kana = &input[to_splitter + 1..to_end];

                    kanji_pos = None;
                    return Some(make_sentence_part(Some(kanji), kana));
                }

                // Find the window of the kana reading for the current kanji
                let kana_window = find_kana_window(&mut char_iter, k_pos)?;

                return Some(make_sentence_part(
                    Some(&input[a_pos..a_pos + a_start.len_utf8()]),
                    &input[kana_window.0..kana_window.1],
                ));
            }
        }

        while let Some(&(p, b)) = char_iter.peek() {
            if b == '[' {
                return Some(make_sentence_part(None, &input[pos..p]));
            }
            char_iter.next()?;
        }

        // input could end with kana
        Some(make_sentence_part(None, &input[pos..]))
    })
}

/// Returns the amount of kanji characters whithin the current kanji frame
fn kanji_count(char_iter: &mut MultiPeek<CharIndices>) -> u32 {
    let mut kanjiparts = 0;
    while let Some((_, c)) = char_iter.peek() {
        kanjiparts += 1;
        if *c == '|' || *c == ']' {
            break;
        }
    }

    char_iter.reset_peek();
    kanjiparts
}

/// Returns the amount of furigana parts
fn furi_count(start: char, char_iter: &mut MultiPeek<CharIndices>) -> u32 {
    let mut furiparts = 0;
    let mut last = start;
    while let Some((_, c)) = char_iter.peek() {
        if *c == ']' {
            // Don't count empty furi
            if last == '|' {
                furiparts -= 1;
            }
            break;
        }

        if *c == '|' {
            furiparts += 1;
        }
        last = *c;
    }
    char_iter.reset_peek();
    furiparts
}

/// Builds a `SentencePartRef` from kanji and kana values. Automatically assigns the passed values correctly
fn make_sentence_part<'a>(kanji: Option<&'a str>, kana: &'a str) -> SentencePartRef<'a> {
    if let Some(kanji) = kanji {
        if kana.is_empty() {
            SentencePartRef::new(kanji)
        } else {
            SentencePartRef::with_kanji(kana, kanji)
        }
    } else {
        SentencePartRef::new(kana)
    }
}

/// Returns the byte range of kana reading for a given kanji(compound)
fn find_kana_window(char_iter: &mut MultiPeek<CharIndices>, k_pos: i8) -> Option<(usize, usize)> {
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
    Some(kana_window)
}
