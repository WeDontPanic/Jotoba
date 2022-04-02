use config::Config;
use criterion::{criterion_group, criterion_main, Criterion};
use search::{
    query::{Query, UserSettings},
    query_parser::QueryParser,
};
use types::jotoba::search::QueryType;

fn get_query(inp: &str, query_type: QueryType) -> Query {
    let settings = UserSettings::default();
    QueryParser::new(inp.to_string(), query_type, settings, 0, 0, true, None)
        .parse()
        .unwrap()
}

fn load() {
    prepare_data(&Config::new(None).expect("config failed"));
}

fn prepare_data(ccf: &Config) {
    rayon::scope(move |s| {
        let cf = ccf.clone();
        s.spawn(move |_| load_resources(&cf));

        //let cf = ccf.clone();
        //s.spawn(move |_| load_suggestions(&cf));

        let cf = ccf.clone();
        s.spawn(move |_| load_indexes(&cf));

        s.spawn(|_| load_tokenizer());
    });
}

pub fn load_tokenizer() {
    use sentence_reader::JA_NL_PARSER;

    // Force parser to parse something to
    // prevent 1. search after launch taking up several seconds
    JA_NL_PARSER.parse("");
}

fn criterion_benchmark(c: &mut Criterion) {
    load();

    c.bench_function("search word", |b| {
        let word = "あ";
        let query = get_query(word, QueryType::Words);
        b.iter(|| {
            let _res = search::word::search(&query).unwrap();
        })
    });
}

pub fn load_resources(config: &Config) {
    resources::initialize_resources(
        config.get_storage_data_path().as_str(),
        config.get_suggestion_sources(),
        config.get_radical_map_path().as_str(),
        config.get_sentences_path().as_str(),
    )
    .expect("Failed to load resources");
}

pub fn load_indexes(config: &Config) {
    search::engine::load_indexes(config).expect("Failed to load v2 index files");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
