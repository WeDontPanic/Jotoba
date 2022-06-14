use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

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

    let tests: Vec<&'static [char]> = vec![&['囗'], &['一'], &['囗', '一'], &['口'], &['口', '一']];
    c.bench_function("Find by radicals", |b| {
        b.iter(|| {
            for i in &tests {
                api::radical::kanji::find_kanji(black_box(i));
            }
        })
    });

    c.bench_function("Find by radicals light", |b| {
        b.iter(|| {
            api::radical::kanji::find_kanji(black_box(&['首']));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
