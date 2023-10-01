use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub use transliteratsiya::{
    transliterator::{FromLatin, ToLatin},
    Armenian, Bulgarian, Russian,
};

const LATIN: &'static str = "Lorem ipsum dolor sit amet";
const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";
const ARMENIAN: &'static str = "Լօրեմ իպսում դօլօր սիտ ամետ";
const RUSSIAN: &'static str = "Лорем ипсум долор сит амет";

fn from_bg(_n: u64) {
    let t = Bulgarian::new();

    let _ = t.to_latin(&BULGARIAN);
}

fn to_bg(_n: u64) {
    let t = Bulgarian::new();

    let _ = t.from_latin(&LATIN);
}

fn from_hy(_n: u64) {
    let t = Armenian::new();

    let _ = t.to_latin(&ARMENIAN);
}

fn to_hy(_n: u64) {
    let t = Armenian::new();

    let _ = t.from_latin(&LATIN);
}

fn from_ru(_n: u64) {
    let t = Russian::new();

    let _ = t.to_latin(&RUSSIAN);
}

fn to_ru(_n: u64) {
    let t = Russian::new();

    let _ = t.from_latin(&LATIN);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_bg", |b| b.iter(|| from_bg(black_box(20))));
    c.bench_function("to_bg", |b| b.iter(|| to_bg(black_box(20))));
    c.bench_function("from_hy", |b| b.iter(|| from_hy(black_box(20))));
    c.bench_function("to_hy", |b| b.iter(|| to_hy(black_box(20))));
    c.bench_function("from_ru", |b| b.iter(|| from_ru(black_box(20))));
    c.bench_function("to_ru", |b| b.iter(|| to_ru(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
