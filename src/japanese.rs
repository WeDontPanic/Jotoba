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

    let mut kanji_readings = kanji_readings(kanji, kana).into_iter();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_kanji2() {
        assert!("𩺊".is_kanji())
    }

    /*
    #[test]
    fn test_furigana_pairs7() {
        let kanji = "新しい酒は古い革袋に入れる";
        let kana = "あたらしいさけはふるいかわぶくろにいれる";

        let result = vec![
            SentencePart {
                kana: String::from("あたら"),
                kanji: Some(String::from("新")),
            },
            SentencePart {
                kana: String::from("しい"),
                kanji: None,
            },
            SentencePart {
                kana: String::from("さけ"),
                kanji: Some(String::from("酒")),
            },
            SentencePart {
                kana: String::from("は"),
                kanji: None,
            },
            SentencePart {
                kana: String::from("ふる"),
                kanji: Some(String::from("古")),
            },
            SentencePart {
                kana: String::from("い"),
                kanji: None,
            },
            SentencePart {
                kana: String::from("革袋"),
                kanji: Some(String::from("かわぶくろ")),
            },
            SentencePart {
                kana: String::from("に"),
                kanji: None,
            },
            SentencePart {
                kana: String::from("い"),
                kanji: Some(String::from("入")),
            },
            SentencePart {
                kana: String::from("れる"),
                kanji: None,
            },
        ];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_kanji_readings() {
        let kanji = "先生はとくに田中を選び出して誉めた";
        let kana = "せんはいはとくにたなかをえらびだしてほめた";
        assert_eq!(
            kanji_readings(kanji, kana),
            vec!["せんはい", "たなか", "えら", "だ", "ほ"]
        )
    }

    #[test]
    fn test_kanji_readings2() {
        let kanji = "先生い";
        let kana = "せんせいい";
        assert_eq!(kanji_readings(kanji, kana), vec!["せんせい"])
    }

    #[test]
    fn test_furigana_pairs33() {
        let kanji = "気持ち";
        let kana = "きもち";

        let result = vec![
            SentencePart {
                kana: "きも".to_string(),
                kanji: Some("気持".to_string()),
            },
            SentencePart {
                kana: "ち".to_string(),
                kanji: None,
            },
        ];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_furigana_pairs0() {
        let kanji = "時々";
        let kana = "ときどき";

        let result = vec![SentencePart {
            kana: kana.to_string(),
            kanji: Some(kanji.to_string()),
        }];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_furigana_pairs() {
        let kanji = "先生はとくに田中を選び出して誉めた";
        let kana = "せんはいはとくにたなかをえらびだしてほめた";

        let result = vec![
            SentencePart {
                kana: "せんはい".to_string(),
                kanji: Some("先生".to_string()),
            },
            SentencePart {
                kana: "はとくに".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "たなか".to_string(),
                kanji: Some("田中".to_string()),
            },
            SentencePart {
                kana: "を".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "えら".to_string(),
                kanji: Some("選".to_string()),
            },
            SentencePart {
                kana: "び".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "だ".to_string(),
                kanji: Some("出".to_string()),
            },
            SentencePart {
                kana: "して".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "ほ".to_string(),
                kanji: Some("誉".to_string()),
            },
            SentencePart {
                kana: "めた".to_string(),
                kanji: None,
            },
        ];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_furigana_pairs6() {
        let kanji = "先生せい";
        let kana = "せんせいせい";

        let result = vec![
            SentencePart {
                kana: "せんせい".to_string(),
                kanji: Some("先生".to_string()),
            },
            SentencePart {
                kana: "せい".to_string(),
                kanji: None,
            },
        ];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }
    #[test]
    fn test_furigana_pairs1() {
        let kanji = "はとくに田中を選び出して誉めた";
        let kana = "はとくにたなかをえらびだしてほめた";

        let result = vec![
            SentencePart {
                kana: "はとくに".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "たなか".to_string(),
                kanji: Some("田中".to_string()),
            },
            SentencePart {
                kana: "を".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "えら".to_string(),
                kanji: Some("選".to_string()),
            },
            SentencePart {
                kana: "び".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "だ".to_string(),
                kanji: Some("出".to_string()),
            },
            SentencePart {
                kana: "して".to_string(),
                kanji: None,
            },
            SentencePart {
                kana: "ほ".to_string(),
                kanji: Some("誉".to_string()),
            },
            SentencePart {
                kana: "めた".to_string(),
                kanji: None,
            },
        ];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_furigana_pairs2() {
        let kanji = "はとくにたなかをえらびだしてほめた";
        let kana = "はとくにたなかをえらびだしてほめた";

        let result = vec![SentencePart {
            kana: kana.clone().to_string(),
            kanji: None,
        }];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_furigana_pairs3() {
        let kanji = "先生";
        let kana = "せんせい";

        let result = vec![SentencePart {
            kana: kana.clone().to_string(),
            kanji: Some(kanji.clone().to_string()),
        }];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_furigana_pairs4() {
        let kanji = "先生いい";
        let kana = "せんせいいい";

        let result = vec![
            SentencePart {
                kana: "せんせい".to_string(),
                kanji: Some("先生".to_string()),
            },
            SentencePart {
                kanji: None,
                kana: "いい".to_string(),
            },
        ];

        assert_eq!(furigana_pairs(kanji, kana), Some(result));
    }

    #[test]
    fn test_get_all_kana_ka_only() {
        let kanji = "きょう";
        assert_eq!(all_words_with_ct(kanji, CharType::Kana), vec![kanji]);
    }

    #[test]
    fn test_get_all_kana_empty() {
        let kanji = "先生頭";
        assert_eq!(all_words_with_ct(kanji, CharType::Kana).len(), 0);
    }

    #[test]
    fn test_get_all_kana() {
        let kanji = "先生はとくに田中を選び出して誉めた";
        assert_eq!(
            all_words_with_ct(kanji, CharType::Kana),
            vec!["はとくに", "を", "び", "して", "めた"]
        );
    }

    #[test]
    fn test_get_all_kanji_ka_only() {
        let kanji = "先生頭";
        assert_eq!(all_words_with_ct(kanji, CharType::Kanji), vec![kanji]);
    }

    #[test]
    fn test_get_all_kanji_empty() {
        let kanji = "せんはいはとくにたなかをえらびだしてほめた";
        assert_eq!(all_words_with_ct(kanji, CharType::Kanji).len(), 0);
    }

    #[test]
    fn test_get_all_kanji() {
        let kanji = "先生はとくに田中を選び出して誉めた";
        assert_eq!(
            all_words_with_ct(kanji, CharType::Kanji),
            vec!["先生", "田中", "選", "出", "誉",]
        );
    }

    #[test]
    fn test_furigana2() {
        let kanji = "今日は";
        let kana = "こんにちは";
        assert_eq!(furigana(kanji, kana), String::from("こんにち"));
    }

    #[test]
    fn test_furigana() {
        let kanji = "今日";
        let kana = "きょう";
        assert_eq!(furigana(kanji, kana), String::from(kana));
    }

    #[test]
    fn test_has_kana() {
        let items: Vec<(&'static str, bool)> = vec![
            ("hallo", false),
            ("コンニチワ", true),
            ("こんいちは", true),
            ("koニチwa", true),
            ("コンnichiハ", true),
            ("コンnichiwa", true),
            ("lol", false),
            ("lolる", true),
        ];

        for item in items {
            assert_eq!(item.0.has_kana(), item.1);
        }
    }

    #[test]
    fn test_is_kana() {
        let items: Vec<(&'static str, bool)> = vec![
            ("hallo", false),
            ("コンニチワ", true),
            ("こんいちは", true),
            ("koニチwa", false),
            ("コンnichiハ", false),
            ("コンnichiwa", false),
            ("lol", false),
            ("lolる", false),
        ];

        for item in items {
            assert_eq!(item.0.is_kana(), item.1);
        }
    }

    #[test]
    fn test_is_kanji() {
        let items: Vec<(&'static str, bool)> = vec![
            ("hallo", false),
            ("コンニチワ", false),
            ("koニチwa", false),
            ("こんnichiは", false),
            ("コンnichiハ", false),
            ("こんいちは", false),
            ("今日は", false),
            ("飛行機", true),
            ("lol", false),
        ];

        for item in items {
            assert_eq!(item.0.is_kanji(), item.1);
        }
    }

    #[test]
    fn test_has_japanese() {
        let items: Vec<(&'static str, bool)> = vec![
            ("hallo", false),
            ("こんにちは", true),
            ("コンニチワ", true),
            ("koニチwa", true),
            ("こんnichiは", true),
            ("コンnichiハ京都", true),
            ("こんいちは", true),
            ("今日は", true),
            ("飛行機", true),
            ("lol今", true),
            ("lol", false),
        ];

        for item in items {
            assert_eq!(item.0.has_japanese(), item.1);
        }
    }

    #[test]
    fn test_is_japanese() {
        let items: Vec<(&'static str, bool)> = vec![
            ("hallo", false),
            ("こんにちは", true),
            ("コンニチワ", true),
            ("koニチwa", false),
            ("こんnichiは", false),
            ("コンnichiハ", false),
            ("こんいちは", true),
            ("今日は", true),
            ("飛行機", true),
            ("lol", false),
        ];

        for item in items {
            assert_eq!(item.0.is_japanese(), item.1);
        }
    }

    #[test]
    fn test_has_kanji() {
        let items: Vec<(&'static str, bool)> = vec![
            ("hallo", false),
            ("コンニチワ", false),
            ("koニチwa", false),
            ("こんnichiは", false),
            ("コンnichiハ", false),
            ("こんいちは", false),
            ("今日は", true),
            ("飛行機", true),
            ("lol", false),
        ];

        for item in items {
            assert_eq!(item.0.has_kanji(), item.1);
        }
    }
    */
}
