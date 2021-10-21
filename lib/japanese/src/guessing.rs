//! add here https://github.com/WeDontPanic/Jotoba/blob/dev/lib/api/src/completions/words/foreign.rs#L21
//! add here https://github.com/WeDontPanic/Jotoba/blob/dev/lib/search/src/word/mod.rs#L291

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
    }

    #[test]
    fn test_fale() {
        test("kind", false);
        test("jinjc", false);
        test("gx", false);
        test("kochen macht spaß", false);
        test("kinderarbeit", false);
        test("buchfuehren", false);
        test("hausaufgaben sind toll", false);
        test("I can't think of proper sentences lol", false);
        test("Mir fallen keine weiteren sätze ein lol", false);
        test("this is a laptop", false);
    }

    fn test(inp: &str, assert: bool) {
        if could_be_romaji(inp) != assert {
            panic!("{} should be {}", inp, assert);
        }
    }
}

fn main() {}

#[derive(Debug)]
enum State {
    FirstMoji,
    SecondMoji(char),
    ThirdMoji(char, char),
}

const SKIP_CHARS: &[char] = &[' ', '　', ',', '、', '。'];

const ALLOW_FIRST: &[char] = &[
    'a', 'e', 'i', 'o', 'u', 'k', 'g', 's', 'z', 't', 'd', 'h', 'b', 'p', 'f', 'n', 'm', 'w', 'r',
    'y', 'j',
];

const ALLOW_SECOND: &[char] = &['a', 'e', 'i', 'o', 'u', 'n', 'y', 'h'];

const ALLOW_THIRD: &[char] = &['a', 'o', 'u'];
const ALLOW_TO_THIRD: &[char] = &['y', 'h'];

/// Returns `true` if input could be romanized japanese text
///
/// Example: "sore wa ongaku desu yo" -> true
/// Example: "this is ugly" -> false
pub fn could_be_romaji(inp: &str) -> bool {
    if inp.is_empty() {
        return false;
    }

    let mut state = State::FirstMoji;
    let mut last_char = inp.chars().next().unwrap();

    for curr_char in inp.chars() {
        let mut first_moji = None;
        let mut second_moji = None;

        match state {
            State::SecondMoji(first) => first_moji = Some(first),
            State::ThirdMoji(first, second) => {
                first_moji = Some(first);
                second_moji = Some(second);
            }
            State::FirstMoji => (),
        }

        let is_skip = SKIP_CHARS.contains(&curr_char);

        if is_skip && ALLOW_SECOND.contains(&last_char) {
            continue;
        } else if is_skip && !ALLOW_SECOND.contains(&last_char) {
            return false;
        }

        let is_first = first_moji.is_none();
        let is_third = second_moji.is_some();

        let curr_in_first = ALLOW_FIRST.contains(&curr_char);
        let curr_in_sec = ALLOW_SECOND.contains(&curr_char);
        let curr_in_third = ALLOW_THIRD.contains(&curr_char);

        if (is_first && !curr_in_first)
            || (!is_first
                && !is_third
                && (!curr_in_sec || is_skip)
                && curr_char != first_moji.unwrap())
            || (is_third && !curr_in_third)
        {
            return false;
        }

        if !is_first && !is_third && curr_in_third && ALLOW_TO_THIRD.contains(&curr_char) {
            state = State::ThirdMoji(first_moji.unwrap(), curr_char);
            last_char = curr_char;
            continue;
        }

        if is_third {
            state = State::FirstMoji;
            last_char = curr_char;
            continue;
        }

        if is_first && curr_in_sec {
            state = State::FirstMoji;
            last_char = curr_char;
            continue;
        }

        if is_first && curr_in_first {
            state = State::SecondMoji(curr_char);
            last_char = curr_char;
            continue;
        }

        state = State::FirstMoji;
        last_char = curr_char;
    }

    ALLOW_SECOND.contains(&last_char)
}
