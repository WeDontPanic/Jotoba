/// Returns true if inp contains at least one katakana character
pub fn has_kana(inp: &str) -> bool {
    inp.chars()
        .into_iter()
        .any(|s| // Hiragana & Katakana
            (s >= '\u{3040}' && s <= '\u{30FF}') || 
             // Half-width katakana
             (s >= '\u{FF66}' && s <= '\u{FF9F}'))
}

/// Returns true if inp contains at least one kana or kanji character
pub fn has_kanji(inp: &str) -> bool {
    has_kana(inp) || 
    inp.chars()
        .into_iter()
        .any(|s| (s >= '\u{3400}' && s <= '\u{4DBF}') || (s >= '\u{4E00}' && s <= '\u{9FFF}')|| (s >= '\u{F900}' && s <= '\u{FAFF}'))
}

pub fn furigana(kanji: &str, kana: &str) -> String{
    let mut new_str = String::from(kana);
    kanji.chars().into_iter().for_each(|c|{
        new_str = new_str.trim_matches(c).to_owned();
    });
    new_str
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
    fn test_has_kanji() {
        let items: Vec<(&'static str, bool)> = vec![
            ("hallo", false),
            ("コンニチワ", true),
            ("koニチwa", true),
            ("こんnichiは", true),
            ("コンnichiハ", true),
            ("こんいちは", true),
            ("今日は", true),
            ("飛行機", true),
            ("lol", false),
        ];

        for item in items {
            assert_eq!(has_kanji(item.0), item.1);
        }
    }
}
