use search::query::{Query, UserSettings};
use types::jotoba::languages::Language;

fn load_data() {
    rayon::scope(|s| {
        s.spawn(|_| {
            resources::load("../../resources/storage_data").unwrap();
        });
        s.spawn(|_| {
            indexes::storage::load("../../indexes").unwrap();
        });
    });
}

#[test]
fn test_search() {
    load_data();
    test_word_search();
}

fn test_word_search() {
    simple_word_search();
}

fn simple_word_search() {
    let query = make_query("musik", Language::German);
    let res = search::word::search(&query).unwrap();
    assert_eq!(res.words().next().unwrap().get_reading().reading, "音楽");
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
