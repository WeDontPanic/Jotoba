use crate::JapaneseExt;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_true() {
        test("shinjitakunakatta", true);
        test("ongakunante", true);
        test("shukudai", true);
        test("akogaredake", true);
        test("daijoubudesuyo", true);
        test("denshaninotteru", true);
        test("sonoshukudaiwokanseishimashitawa", true);
        test("korewanagaibunshodayone", true);
        test("atarashiibunwokangaenai", true);
        test("atarashiibunwokangaenakute", true);
        test("shinjitai", true);
        test("ongaku", true);
        test("sore wa ongaku desu yo", true);
        test("kirishima", true);
        test("deine oma", true);
        test("kyotou", true);
        test("onsen", true);
        test("onsei", true);
        test("otagai", true);
        test("kansei", true);
        test("kanpeki", true);
        test("fuben", true);
        test("kansetsu", true);
        test("chokusetsu", true);
    }

    #[test]
    fn test_fale() {
        test("kind", false);
        test("jinjc", false);
        test("gx", false);
        test("kochen macht spaß", false);
        test("kinderarbeit", false);
        test("hausaufgaben sind toll", false);
        test("I can't think of proper sentences lol", false);
        test("Mir fallen keine weiteren sätze ein lol", false);
        test("this is a laptop", false);
    }

    fn test(inp: &str, assert: bool) {
        if could_be_romaji(inp) != assert {
            panic!("{:?} should be {}", inp, assert);
        }
    }
}

/// Returns `true` if input could be romanized japanese text
///
/// Example: "sore wa ongaku desu yo" -> true
/// Example: "this is ugly" -> false
pub fn could_be_romaji(inp: &str) -> bool {
    is_romaji_repl(inp).is_some()
}

pub fn is_romaji_repl(inp: &str) -> Option<String> {
    let mut inp = inp.to_string();
    let to_replace = &['.', '(', ')', '、', '。', '「', '」', ' ', '\'', '"'];
    for to_repl in to_replace {
        inp = inp.replace(*to_repl, "");
    }
    inp.to_hiragana().is_japanese().then(|| inp)
}
