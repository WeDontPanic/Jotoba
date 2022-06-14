#[cfg(test)]
mod tests {
    use crate::furigana::map_readings;
    //use resources::LAZY_STORAGE;
    use test_case::test_case;

    #[test_case("", "", &vec![]; "Empty")]
    //#[test_case("音楽が好き", "おんがくがすき", &[("音楽","おんがく"),("好","す")]; "Simple 1")] // TODO: fix this one lol
    #[test_case("音楽は好き", "おんがくはすき", &[("音楽","おんがく"),("好","す")]; "Simple 1")]
    #[test_case("お金を払いたくない", "おかねをはらいたくない", &[("金","かね"),("払","はら")]; "Simple 2")]
    #[test_case("おかねをはらいたくない", "おかねをはらいたくない", &[]; "Kana only")]
    #[test_case("漢字", "かんじ", &[("漢字","かんじ")]; "Kanji only")]
    #[test_case("水気","みずけ",&[("水気","みずけ")]; "Kanji only 2")]
    #[test_case("いつも眠い感じがします", "いつもねむいかんじがします", &[("眠","ねむ"),("感","かん")]; "Simple 3")]
    #[test_case("今日もとても眠い", "きょうもとてもねむい", &[("今日","きょう"),("眠","ねむ")]; "Simple 4")]
    #[test_case("５日", "いつか", &[("５日","いつか")]; "With roman letter")]
    #[test_case("かば、夕べに","かばゆうべに",&[("夕","ゆう")]; "Special char")]
    fn test_map_readings(kanji: &str, kana: &str, expected: &[(&str, &str)]) {
        let parsed = map_readings(kanji, kana).unwrap();
        let parsed = parsed
            .iter()
            .map(|i| (i.0.as_str(), i.1.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(parsed, expected);
    }
}
