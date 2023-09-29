use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub use transliteratsiya::transliterator::{Bulgarian, FromLatin, ToLatin};

const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";

fn bg(_n: u64) {
    let t = Bulgarian::new();

    let _ = t.to_latin(&BULGARIAN);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bg", |b| b.iter(|| bg(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
