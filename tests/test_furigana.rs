use japanese::furigana::generate;
use resources::LAZY_STORAGE;
use test_case::test_case;

// TODO: fix those
//#[test_case("音楽が大好き","おんがくがだいすき","[音楽|おん|がく]が[大好|だい|す]き"; "Simple")] // TODO FIX this one
#[test_case("待合","まちあい","[待合|まち|あい]"; "Simple 9")]
#[test_case("","",""; "Empty")]
#[test_case("音楽は好き","おんがくはすき","[音楽|おん|がく]は[好|す]き"; "Simple 2")]
#[test_case("携帯を見つけられない","けいたいをみつけられない","[携帯|けい|たい]を[見|み]つけられない"; "Simple 3")]
#[test_case("全部曖昧にして","ぜんぶあいまいにして","[全部曖昧|ぜん|ぶ|あい|まい]にして"; "Simple 4")]
#[test_case("正しくなくても意味がなくても","ただしくなくてもいみがなくても","[正|ただ]しくなくても[意味|い|み]がなくても"; "Simple 5")]
#[test_case("音楽に合わせて踊る","おんがくにあわせておどる","[音楽|おん|がく]に[合|あ]わせて[踊|おど]る"; "Simple 6")]
#[test_case("私の趣味は音楽だ","わたしのしゅみはおんがくだ","[私|わたし]の[趣味|しゅ|み]は[音楽|おん|がく]だ"; "Simple 7")]
#[test_case("趣味のいい","しゅみのいい","[趣味|しゅ|み]のいい"; "Simple 8")]
#[test_case("話したくない","はなしたくない","[話|はな]したくない"; "One kanji")]
#[test_case("音楽教室","おんがくきょうしつ","[音楽教室|おん|がく|きょう|しつ]"; "Kanji only")]
#[test_case("だいがくにかよってる","だいがくにかよってる","だいがくにかよってる"; "Kana only")]
#[test_case("朝に道を聞かば、夕べに死すとも可なり","あしたにみちをきかばゆうべにしすともかなり","[朝|あした]に[道|みち]を[聞|き]かば、[夕|ゆう]べに[死|し]すとも[可|か]なり"; "Special character")]
#[test_case("水気","みずけ","[水気|みず|け]"; "Weird reading 1")]
fn test_gen_furigana(kanji: &str, kana: &str, expected: &str) {
    let retrieve: resources::retrieve::kanji::KanjiRetrieve<'_> = LAZY_STORAGE.kanji();
    let built = generate::checked(retrieve, kanji, kana);
    assert_eq!(built.as_str(), expected);
}

#[test_case("会話","かいわ",&[("会","かい"),("話","わ")])]
#[test_case("方向音痴","ほうこうおんち", &[("方","ほう"),("向","こう"),("音","おん"),("痴","ち")])]
#[test_case("水気","みずけ",&[("水","みず"), ("気","け")])]
// TODO: fix this
//#[test_case("今日の一針明日の十針","きょうのひとはりあすのとはり",&[("今日","きょう"), ("一","ひと"),("針","はり"),("明日","あす"),("十","と"),("針","はり")])]
fn test_assign_reading(kanji: &str, kana: &str, expected: &[(&str, &str)]) {
    let retrieve: resources::retrieve::kanji::KanjiRetrieve<'_> = LAZY_STORAGE.kanji();
    let built = generate::assign_readings(retrieve, kanji, kana);

    let built_cmp = built.as_ref().map(|b| {
        b.iter()
            .map(|i| (i.0.as_str(), i.1.as_str()))
            .collect::<Vec<_>>()
    });

    assert_eq!(built_cmp.as_ref().map(|i| i.as_slice()), Some(expected));
}
