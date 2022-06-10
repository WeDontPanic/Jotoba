use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn load() {
    resources::load("../resources/storage_data").unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    load();

    c.bench_function("Get Kanji", |b| {
        b.iter(|| {
            //let  = resources::get().words();
            resources::get().kanji().by_literal(black_box('跡'));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
