use itertools::Itertools;

/// Returns true if inp contains at least one kana character
pub fn char_is_kana(s: char) -> bool {
    s >= '\u{3040}' && s <= '\u{30FF}' || s >= '\u{FF66}' && s <= '\u{FF9F}'
}

/// Returns true if inp contains at least one kana character
pub fn char_is_kanji(s: char) -> bool {
    (s >= '\u{3400}' && s <= '\u{4DBF}')
        || (s >= '\u{4E00}' && s <= '\u{9FFF}')
        || (s >= '\u{F900}' && s <= '\u{FAFF}' || s == '\u{3005}')
}

/// Returns true if s is of type ct
pub fn char_is_of_type(s: char, ct: CharType) -> bool {
    get_char_type(s) == ct
}

/// Get the CharType of a character
pub fn get_char_type(s: char) -> CharType {
    if char_is_kana(s) {
        CharType::Kana
    } else if char_is_kanji(s) {
        CharType::Kanji
    } else {
        CharType::Other
    }
}

/// Returns true if inp contains at least one kana character
pub fn has_kana(inp: &str) -> bool {
    inp.chars().into_iter().any(|s| char_is_kana(s))
}

/// Returns true if inp is entirely written in kana
pub fn is_kana(inp: &str) -> bool {
    !inp.chars().into_iter().any(|s| !char_is_kana(s))
}

/// Returns true if inp is entirely written with kanji
pub fn is_kanji(inp: &str) -> bool {
    !inp.chars().into_iter().any(|s| !char_is_kanji(s))
}

/// Returns true if inp has at least one kanji
pub fn has_kanji(inp: &str) -> bool {
    inp.chars().into_iter().any(|s| char_is_kanji(s))
}

/// Returns true if inp is build with kanji and kana only
pub fn is_japanese(inp: &str) -> bool {
    let mut buf = [0; 16];
    !inp.chars().into_iter().any(|c| {
        let s = c.encode_utf8(&mut buf);
        !is_kana(&s) && !is_kanji(&s)
    })
}

/// Returns true if inp contains japanese characters
pub fn has_japanese(inp: &str) -> bool {
    let mut buf = [0; 16];
    inp.chars().into_iter().any(|c| {
        let s = c.encode_utf8(&mut buf);
        is_kana(&s) || is_kanji(&s)
    })
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
    if !is_kana(kana) || !is_japanese(kanji) || kanji.is_empty() || kana.is_empty() {
        return None;
    }

    let mut kanji_readings = kanji_readings(kanji, kana).into_iter();

    let mut parts: Vec<SentencePart> = Vec::new();
    let mut last_char_type: Option<CharType> = None;

    let mut word_buf = String::new();

    for curr_char in kanji.chars() {
        let curr_char_type = get_char_type(curr_char);

        if last_char_type.is_some() && last_char_type.unwrap() != curr_char_type {
            // If char type changes
            let part = SentencePart {
                kana: {
                    if last_char_type.unwrap() == CharType::Kana {
                        word_buf.clone()
                    } else {
                        kanji_readings.next().unwrap()
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
                kanji_readings.next().unwrap()
            }
        },
        kanji: (last_char_type.unwrap() == CharType::Kanji).then(|| word_buf.clone()),
    };
    parts.push(part);

    Some(parts)
}

/// Return all words of chartype ct
pub fn all_words_with_ct(inp: &str, ct: CharType) -> Vec<String> {
    let mut all: Vec<String> = Vec::new();
    let mut curr = String::new();
    let mut iter = inp.chars().into_iter();
    while let Some(c) = iter.next() {
        if char_is_of_type(c, ct) {
            curr.push(c);
            continue;
        } else {
            if !curr.is_empty() {
                all.push(curr.clone());
            }
            curr.clear();
            iter.take_while_ref(|i| !char_is_of_type(*i, ct)).count();
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
        // TODO need to replace backwards!!
        kana_mod = kana_mod.replacen(&ka_kana, " ", 1);
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

#[cfg(test)]
mod test {
    use super::*;

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

        let result = vec![SentencePart {
            kana: kana.to_string(),
            kanji: Some(kanji.to_string()),
        }];

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
            assert_eq!(has_kana(item.0), item.1);
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
            assert_eq!(is_kana(item.0), item.1);
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
            assert_eq!(is_kanji(item.0), item.1);
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
            assert_eq!(has_japanese(item.0), item.1);
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
            assert_eq!(is_japanese(item.0), item.1);
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
            assert_eq!(has_kanji(item.0), item.1);
        }
    }
}
