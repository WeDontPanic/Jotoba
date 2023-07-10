use criterion::{criterion_group, criterion_main, Criterion};
use search::{
    executor::SearchExecutor,
    query::{parser::QueryParser, Query, UserSettings},
    word,
};
use types::jotoba::{language::Language, search::SearchTarget};

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn get_query(inp: &str, query_type: SearchTarget) -> Query {
    let mut settings = UserSettings::default();
    settings.user_lang = Language::German;
    settings.show_english = true;
    QueryParser::new(inp.to_string(), query_type, settings)
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

    c.bench_function("search word: kanji", |b| {
        let query = get_query("kanji", SearchTarget::Words);
        b.iter(|| search(&query))
    });

    c.bench_function("search word: jp", |b| {
        let query = get_query("おはよう", SearchTarget::Words);
        b.iter(|| search(&query))
    });

    c.bench_function("search kanji reading", |b| {
        let query = get_query("事 ジ", SearchTarget::Words);
        b.iter(|| search(&query))
    });
}

#[inline]
fn search(query: &Query) {
    let _res = SearchExecutor::new(word::Search::new(&query)).run();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
