use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub use transliteratsiya::{
    transliterator::{FromLatin, ToLatin},
    Bulgarian,
};

const LATIN: &'static str = "Lorem ipsum dolor sit amet";
const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";

fn from_bg(_n: u64) {
    let t = Bulgarian::new();

    let _ = t.to_latin(&BULGARIAN);
}

fn to_bg(_n: u64) {
    let t = Bulgarian::new();

    let _ = t.from_latin(&LATIN);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_bg", |b| b.iter(|| from_bg(black_box(20))));
    c.bench_function("to_bg", |b| b.iter(|| to_bg(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
