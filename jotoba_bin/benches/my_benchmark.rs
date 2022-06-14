use criterion::{criterion_group, criterion_main, Criterion};
use search::query::{parser::QueryParser, Query, UserSettings};
use types::jotoba::{languages::Language, search::QueryType};

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn get_query(inp: &str, query_type: QueryType) -> Query {
    let mut settings = UserSettings::default();
    settings.user_lang = Language::German;
    settings.show_english = true;
    QueryParser::new(inp.to_string(), query_type, settings, 0, 0, true, None)
        .parse()
        .unwrap()
}

fn load() {
    rayon::scope(move |s| {
        s.spawn(move |_| {
            resources::load("../resources/storage_data").unwrap();
        });
        s.spawn(move |_| {
            indexes::storage::load("../resources/indexes").unwrap();
        });
        s.spawn(|_| {
            // load ja nl parser since its lazy
            sentence_reader::load_parser("../resources/unidic-mecab")
        });
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    load();

    c.bench_function("search word", |b| {
        let query = get_query("あ", QueryType::Words);
        b.iter(|| {
            let _res = search::word::search(&query).unwrap();
        })
    });

    c.bench_function("search kanji reading", |b| {
        let query = get_query("事 ジ", QueryType::Words);
        b.iter(|| {
            let _ = search::word::search(&query).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
