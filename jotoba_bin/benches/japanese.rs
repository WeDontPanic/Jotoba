use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Fullwidth to HW", |b| {
        b.iter(|| {
            japanese::to_halfwidth(black_box("１２３４！＠＃＄A＆Ｒｕｓｔ－１．６！"));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
