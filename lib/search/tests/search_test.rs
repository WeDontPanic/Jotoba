use search::query::{parser::QueryParser, Query, UserSettings};
use test_case::test_case;
use types::jotoba::{languages::Language, search::QueryType, words::part_of_speech::PosSimple};

/// Loads the data. Always has to be called
#[test]
fn load_search() {
    load_data();
}

/// ----------- Inflections --------------- ///

// TODO
//

///
/// ----------- Sentence reader --------------- ///

// TODO
//

///
/// ----------- Kanji (right) --------------- ///

// TODO
//

//
/// ----------- Direkt on top --------------- ///

// TODO
//

/// ----------- Simple search ------------- ///

#[test_case("musik", Language::German, "音楽")]
#[test_case("音楽", Language::German, "音楽")]
#[test_case("to sleep", Language::English, "寝る")]
#[test_case("買う", Language::English, "買う")]
#[test_case("know", Language::German, "知る"; "Find in english too")]
#[test_case("remember", Language::German, "覚える"; "Find in english too 2")]
#[test_case("think", Language::German, "思う"; "Find in english too 3")]
#[test_case("especially", Language::German, "特に"; "Find in english too 4")]
fn word_search(query_str: &str, language: Language, first_res: &str) {
    wait();

    let query = make_query(query_str, language);
    let res = search::word::search(&query).unwrap();
    let word = match res.words().next() {
        Some(n) => n,
        None => return,
    };

    if !word.has_reading(first_res) {
        panic!("Expected {query_str:?} ({language}) to return {first_res:?} as first result (but was: {:?})", word.get_reading().reading);
    }
}

/// ------------- Part of speech filter ----------- ///

#[test_case("音楽 #adjective", &[PosSimple::Adjective], &["音楽的", "標題音楽", "電子音楽"]; "Test single tag")]
#[test_case("speak #verb", &[PosSimple::Verb], &["話す","話せる"]; "Test foreign inp")]
#[test_case("speak #noun", &[PosSimple::Noun], &["言葉"]; "Test unlikely")]
fn pos_tag_test(query_str: &str, exp_pos: &[PosSimple], exp_res: &[&str]) {
    wait();

    let query = parse_query(query_str, Language::English, QueryType::Words);
    let res = search::word::search(&query).expect("Search crashed");
    let have_tag = res
        .words()
        .all(|i| exp_pos.iter().all(|j| i.has_pos(&[*j])));
    assert!(have_tag);
    assert!(exp_res
        .iter()
        .all(|j| res.words().any(|w| w.has_reading(j))));
}

/// ----------- JP search Relevance ----------- ///

#[test]
fn test_jp_search() {
    wait();

    // Expect most important word on top
    for word in resources::get().words().iter().step_by(317) {
        let reading = &word.get_reading().reading;
        word_search(reading, Language::Swedish, reading);
    }
}

// ------------ Romaji search ---------------- ///

#[test_case("kore",&["これ"])]
#[test_case("tokasu", &["溶かす"])]
#[test_case("kisuu", &["奇数"])]
#[test_case("daijoubu", &["大丈夫"])]
#[test_case("jikan", &["時間"])]
#[test_case("kono", &["この"])]
#[test_case("kanjiru", &["感じる"])]
#[test_case("ongaku", &["音楽"])]
#[test_case("kimi", &["君"])]
//#[test_case("kiku", &["聞く"])]
//#[test_case("suki", &["好き"])]
fn test_romaji(query_str: &str, expected: &[&str]) {
    wait();

    let res =
        search::word::search(&make_query(query_str, Language::English)).expect("Engine failed");
    for exp in expected.iter() {
        if !res.words().take(3).any(|i| i.has_reading(exp)) {
            panic!("Expected {:?} to find {exp:?} (Romaji search)", query_str);
        }
    }
}

fn make_query(query_str: &str, language: Language) -> Query {
    Query {
        query: query_str.to_string(),
        settings: UserSettings {
            user_lang: language,
            ..UserSettings::default()
        },
        ..Query::default()
    }
}

fn parse_query(query_str: &str, language: Language, q_type: QueryType) -> Query {
    let mut settings = UserSettings::default();
    settings.user_lang = language;
    QueryParser::new(query_str.to_string(), q_type, settings, 0, 0, true, None)
        .parse()
        .expect("Invaild query passed")
}

fn load_data() {
    if resources::is_loaded() || indexes::storage::is_loaded() {
        return;
    }
    rayon::scope(|s| {
        s.spawn(|_| {
            resources::load("../../resources/storage_data").unwrap();
        });
        s.spawn(|_| {
            indexes::storage::load("../../indexes").unwrap();
        });
    });
}

fn wait() {
    indexes::storage::wait();
    resources::wait();
}
