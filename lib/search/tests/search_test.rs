use search::query::{Query, UserSettings};
use types::jotoba::languages::Language;

fn load_data() {
    indexes::storage::load("../../indexes").unwrap();
    resources::load("../../resources/storage_data").unwrap();
}

#[test]
fn test_simple_word_search() {
    load_data();

    let query = make_query("musik", Language::German);
    let res = search::word::search(&query).unwrap();

    assert_eq!(res.words().next().unwrap().get_reading().reading, "音楽");

    let resources = resources::get();
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
