use std::iter::Peekable;

use itertools::Itertools;

pub trait JapaneseExt {
    /// Returns true if self is of type ct
    fn is_of_type(&self, ct: CharType) -> bool;

    /// Get the CharType of a character
    fn get_text_type(&self) -> CharType;

    /// Returns true if self contains at least one kana character
    fn has_kana(&self) -> bool;

    /// Returns true if self is entirely written in kana
    fn is_kana(&self) -> bool;

    /// Returns true if inp is entirely written with kanji
    fn is_kanji(&self) -> bool;

    /// Returns true if inp has at least one kanji
    fn has_kanji(&self) -> bool;

    /// Returns true if inp is build with kanji and kana only
    fn is_japanese(&self) -> bool;

    /// Returns true if inp contains japanese characters
    fn has_japanese(&self) -> bool;

    /// Returns true if self is written in katakana
    fn is_katakana(&self) -> bool;

    /// Returns true if self is written in hiragana
    fn is_hiragana(&self) -> bool;

    /// Returns the amount of kanji self has
    fn kanji_count(&self) -> usize;
}

impl JapaneseExt for char {
    fn is_katakana(&self) -> bool {
        (*self) >= '\u{30A0}' && (*self) <= '\u{30FF}'
    }

    fn is_hiragana(&self) -> bool {
        (*self) >= '\u{3040}' && (*self) <= '\u{309F}'
    }

    fn is_kana(&self) -> bool {
        self.is_hiragana() || self.is_katakana()
    }

    fn is_kanji(&self) -> bool {
        ((*self) >= '\u{3400}' && (*self) <= '\u{4DBF}')
            || ((*self) >= '\u{4E00}' && (*self) <= '\u{9FFF}')
            || ((*self) >= '\u{F900}' && (*self) <= '\u{FAFF}')
            || ((*self) >= '\u{FF10}' && (*self) <= '\u{FF19}')
            || (*self) == '\u{3005}'
            || (*self) == '\u{29E8A}'
    }

    fn has_kana(&self) -> bool {
        return self.is_kana();
    }

    fn has_kanji(&self) -> bool {
        self.is_kanji()
    }

    fn is_of_type(&self, ct: CharType) -> bool {
        self.get_text_type() == ct
    }

    fn get_text_type(&self) -> CharType {
        if self.is_kana() {
            CharType::Kana
        } else if self.is_kanji() {
            CharType::Kanji
        } else {
            CharType::Other
        }
    }

    fn is_japanese(&self) -> bool {
        self.is_kana() || self.is_kanji()
    }

    fn has_japanese(&self) -> bool {
        self.is_japanese()
    }

    fn kanji_count(&self) -> usize {
        if self.is_kanji() {
            1
        } else {
            0
        }
    }
}

impl JapaneseExt for str {
    fn is_of_type(&self, ct: CharType) -> bool {
        self.get_text_type() == ct
    }

    fn get_text_type(&self) -> CharType {
        if self.is_kanji() {
            CharType::Kanji
        } else if self.is_kana() {
            CharType::Kana
        } else {
            CharType::Other
        }
    }

    fn is_hiragana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_hiragana())
    }

    fn is_katakana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_katakana())
    }

    fn has_kana(&self) -> bool {
        self.chars().into_iter().any(|s| s.is_kana())
    }

    fn is_kana(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_kana())
    }

    fn is_kanji(&self) -> bool {
        !self.chars().into_iter().any(|s| !s.is_kanji())
    }

    fn has_kanji(&self) -> bool {
        self.chars().into_iter().any(|s| s.is_kanji())
    }

    fn is_japanese(&self) -> bool {
        let mut buf = [0; 16];
        !self.chars().into_iter().any(|c| {
            let s = c.encode_utf8(&mut buf);
            !s.is_kana() && !s.is_kanji()
        })
    }

    fn has_japanese(&self) -> bool {
        let mut buf = [0; 16];
        self.chars().into_iter().any(|c| {
            let s = c.encode_utf8(&mut buf);
            s.is_kana() || s.is_kanji()
        })
    }

    fn kanji_count(&self) -> usize {
        self.chars().into_iter().filter(|i| i.is_kanji()).count()
    }
}

impl JapaneseExt for Vec<char> {
    fn is_of_type(&self, ct: CharType) -> bool {
        self.get_text_type() == ct
    }

    fn get_text_type(&self) -> CharType {
        if self.is_kanji() {
            CharType::Kanji
        } else if self.is_kana() {
            CharType::Kana
        } else {
            CharType::Other
        }
    }

    fn is_hiragana(&self) -> bool {
        !self.iter().any(|s| !s.is_hiragana())
    }

    fn is_katakana(&self) -> bool {
        !self.iter().any(|s| !s.is_katakana())
    }

    fn has_kana(&self) -> bool {
        self.iter().any(|s| s.is_kana())
    }

    fn is_kana(&self) -> bool {
        !self.iter().any(|s| !s.is_kana())
    }

    fn is_kanji(&self) -> bool {
        !self.iter().any(|s| !s.is_kanji())
    }

    fn has_kanji(&self) -> bool {
        self.iter().any(|s| s.is_kanji())
    }

    fn is_japanese(&self) -> bool {
        let mut buf = [0; 16];
        !self.iter().any(|c| {
            let s = c.encode_utf8(&mut buf);
            !s.is_kana() && !s.is_kanji()
        })
    }

    fn has_japanese(&self) -> bool {
        let mut buf = [0; 16];
        self.iter().any(|c| {
            let s = c.encode_utf8(&mut buf);
            s.is_kana() || s.is_kanji()
        })
    }

    fn kanji_count(&self) -> usize {
        self.iter().filter(|i| i.is_kanji()).count()
    }
}

pub fn furigana(kanji: &str, kana: &str) -> String {
    let mut new_str = String::from(kana);
    kanji.chars().into_iter().for_each(|c| {
        new_str = new_str.trim_matches(c).to_owned();
    });
    new_str
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharType {
    Kana,
    Kanji,
    Other,
}

/// Create SentenceParts out of an input sencence which has to be passed
/// once written in kanji and once in kana characters
pub fn furigana_pairs(kanji: &str, kana: &str) -> Option<Vec<SentencePart>> {
    if !kana.is_kana() || !kanji.is_japanese() || kanji.is_empty() || kana.is_empty() {
        return None;
    }

    //let mut kanji_readings = kanji_readings(kanji, kana).into_iter();
    let mut kanji_readings = furi_algo(kanji, kana).into_iter();

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
                kanji: (last_char_type.unwrap() == CharType::Kanji).then(|| word_buf.clone()),
            };
            parts.push(part);
            word_buf.clear();
        }

        word_buf.push(curr_char);

        last_char_type = Some(curr_char_type);
    }

    let part = SentencePart {
        kana: {
            if last_char_type.unwrap() == CharType::Kana {
                word_buf.clone()
            } else {
                kanji_readings.next().unwrap_or_default()
            }
        },
        kanji: (last_char_type.unwrap() == CharType::Kanji).then(|| word_buf.clone()),
    };
    parts.push(part);
    Some(parts)
}

/// Replacen but backwards
fn replacen_backwards(inp: &str, from: &str, to: &str, count: usize) -> String {
    reverse_str(&reverse_str(inp).replacen(&reverse_str(from), &reverse_str(to), count))
}

/// Retuns the input string reversed
fn reverse_str<S: AsRef<str>>(inp: S) -> String {
    inp.as_ref().chars().into_iter().rev().collect()
}

/// Return all words of chartype ct
pub fn all_words_with_ct(inp: &str, ct: CharType) -> Vec<String> {
    let mut all: Vec<String> = Vec::new();
    let mut curr = String::new();
    let mut iter = inp.chars().into_iter();
    while let Some(c) = iter.next() {
        if c.is_of_type(ct) {
            curr.push(c);
            continue;
        } else {
            if !curr.is_empty() {
                all.push(curr.clone());
            }
            curr.clear();
            iter.take_while_ref(|i| !i.is_of_type(ct)).count();
        }
    }
    if !curr.is_empty() {
        all.push(curr.clone());
    }
    all
}

/// Create SentenceParts out of an input sencence which has to be passed
/// once written in kanji and once in kana characters
pub fn kanji_readings(kanji: &str, kana: &str) -> Vec<String> {
    let all_kana = all_words_with_ct(kanji, CharType::Kana);

    let mut kana_mod = kana.clone().to_string();
    for ka_kana in all_kana {
        //kana_mod = kana_mod.replacen(&ka_kana, " ", 1);
        if let Some(_pos) = kana_mod.find(&ka_kana) {
            kana_mod = replacen_backwards(&kana_mod, &ka_kana, " ", 1);
        }
    }

    kana_mod
        .split(" ")
        .filter_map(|i| (!i.is_empty()).then(|| i.to_string()))
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub struct SentencePart {
    pub kana: String,
    pub kanji: Option<String>,
}

impl SentencePart {
    /// Make the kana reading good looking as furigana text
    /// If the kanji count matches with kana count, a space will
    /// be added between each char
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

pub fn furi_algo(kanji: &str, kana: &str) -> Vec<String> {
    let mut kanji_iter = kanji.chars().into_iter().peekable();
    let kana = kana.chars().into_iter().collect::<Vec<_>>();

    let mut kana_pos = strip_until_kanji(&mut kanji_iter);

    let mut result: Vec<String> = Vec::new();

    let mut curr_kanji = Vec::new();
    loop {
        if kana_pos >= kana.len() {
            break;
        }

        // Kana from current position to end
        let curr_kana = &kana[kana_pos..];

        // Get all chars until next kanji
        let (part_kana, part_kanji) = to_next_kanji(&mut kanji_iter);

        // If last part is kanji only take rest of kana reading
        if part_kana.is_empty() {
            result.push(curr_kana.into_iter().collect());
            break;
        }

        // Current kanji buff
        curr_kanji.clear();
        let found = loop {
            curr_kanji.push(kana[kana_pos]);
            kana_pos += 1;

            if starts_with(&curr_kana, &curr_kanji, &part_kana) {
                break true;
            }

            if curr_kanji.len() >= curr_kana.len() || kana_pos >= kana.len() {
                break false;
            }
        };

        if !found {
            // Error
            println!("error");
            return vec![];
        }

        result.push(char_arr_to_string(&curr_kanji));

        for _ in 0..(part_kana.len() + part_kanji.len()) {
            kanji_iter.next();
        }

        kana_pos += part_kana.len();
    }

    //println!("out: {:?}", result);
    result
}

/// Checks whether 'arr' starts with a+b
fn starts_with<T>(arr: &[T], a: &[T], b: &[T]) -> bool
where
    T: PartialEq,
{
    if a.len() + b.len() > arr.len() {
        return false;
    }

    for (pos, item) in a.iter().enumerate() {
        if arr[pos] != *item {
            return false;
        }
    }

    for (pos, item) in b.iter().enumerate() {
        if arr[pos + a.len()] != *item {
            return false;
        }
    }

    true
}

fn char_arr_to_string(vec: &Vec<char>) -> String {
    vec.iter().collect()
}

fn to_next_kanji<T>(kanji_iter: &mut Peekable<T>) -> (Vec<char>, Vec<char>)
where
    T: Iterator<Item = char> + Clone,
{
    let mut kanji_iter = kanji_iter.clone();
    let kanji = kanji_iter
        .take_while_ref(|i| i.is_kanji())
        .collect::<Vec<_>>();
    let kana = kanji_iter
        .take_while_ref(|i| i.is_kana())
        .collect::<Vec<_>>();
    (kana, kanji)
}

fn strip_until_kanji<T>(kanji_iter: &mut Peekable<T>) -> usize
where
    T: Iterator<Item = char>,
{
    let mut i = 0;
    loop {
        if kanji_iter.peek().map(|i| i.is_kanji()).unwrap_or(true) {
            break i;
        }

        kanji_iter.next();
        i += 1;
    }
}
