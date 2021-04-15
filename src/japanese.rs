/// Returns true if inp contains at least one kana character
pub fn has_kana(inp: &str) -> bool {
    inp.chars()
        .into_iter()
        .any(|s| // Hiragana & Katakana
            (s >= '\u{3040}' && s <= '\u{30FF}' || 
             // Half-width katakana
             s >= '\u{FF66}' && s <= '\u{FF9F}'))
}

/// Returns true if inp is entirely written in kana
pub fn is_kana(inp: &str) -> bool {
    !inp.chars()
        .into_iter()
        .any(|s| (s <= '\u{3040}' || s >= '\u{30FF}') && (s <= '\u{FF66}' || s >= '\u{FF9F}'))
}

/// Returns true if inp is entirely written with kanji
pub fn is_kanji(inp: &str) -> bool {
    !inp.chars()
        .into_iter()
        .any(|s| (s <= '\u{3400}' || s >= '\u{4DBF}') && (s <= '\u{4E00}' || s >= '\u{9FFF}') && (s <= '\u{F900}' || s >= '\u{FAFF}'))
}

/// Returns true if inp has at least one kanji
pub fn has_kanji(inp: &str) -> bool {
    inp.chars()
        .into_iter()
        .any(|s| (s >= '\u{3400}' && s <= '\u{4DBF}') || (s >= '\u{4E00}' && s <= '\u{9FFF}')|| (s >= '\u{F900}' && s <= '\u{FAFF}'))
}


/// Returns true if inp is build with kanji and kana only
pub fn is_japanese(inp: &str) -> bool {
    let mut buf = [0;16];
    !inp.chars().into_iter().any(|c|{
        let s = c.encode_utf8(&mut buf);
        !is_kana(&s) && !is_kanji(&s)
    })
}

/// Returns true if inp contains japanese characters
pub fn has_japanese(inp: &str) -> bool {
    let mut buf = [0;16];
    inp.chars().into_iter().any(|c|{
        let s = c.encode_utf8(&mut buf);
        is_kana(&s) || is_kanji(&s)
    })
}

pub fn furigana(kanji: &str, kana: &str) -> String{
    let mut new_str = String::from(kana);
    kanji.chars().into_iter().for_each(|c|{
        new_str = new_str.trim_matches(c).to_owned();
    });
    new_str
}

pub fn furigana_pairs(kanji: &str, kana: &str) -> Vec<SentencePart>{
    vec![]
}

#[derive(Debug, Clone, PartialEq)]
pub struct SentencePart{
    kana: String,
    kanji: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_furigana2(){
        let kanji = "今日は";
        let kana = "こんにちは";
        assert_eq!(furigana(kanji, kana), String::from("こんにち"));
    }

    #[test]
    fn test_furigana(){
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
