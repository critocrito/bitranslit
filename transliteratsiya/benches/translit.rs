use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub use transliteratsiya::{transliterate, Language};

const LATIN: &'static str = "Lorem ipsum dolor sit amet";
const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";
const ARMENIAN: &'static str = "Լօրեմ իպսում դօլօր սիտ ամետ";
const RUSSIAN: &'static str = "Лорем ипсум долор сит амет";
const UKRANIAN: &'static str = "Лорем іпсум долор сіт амет";

fn from_bg(_n: u64) {
    let _ = transliterate(&BULGARIAN, Language::Bulgarian, false);
}

fn to_bg(_n: u64) {
    let _ = transliterate(&LATIN, Language::Bulgarian, true);
}

fn from_hy(_n: u64) {
    let _ = transliterate(&ARMENIAN, Language::Armenian, false);
}

fn to_hy(_n: u64) {
    let _ = transliterate(&LATIN, Language::Armenian, true);
}

fn from_ru(_n: u64) {
    let _ = transliterate(&RUSSIAN, Language::Russian, false);
}

fn to_ru(_n: u64) {
    let _ = transliterate(&LATIN, Language::Russian, true);
}

fn from_ua(_n: u64) {
    let _ = transliterate(&UKRANIAN, Language::Ukranian, false);
}

fn to_ua(_n: u64) {
    let _ = transliterate(&LATIN, Language::Ukranian, true);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_bg", |b| b.iter(|| from_bg(black_box(20))));
    c.bench_function("to_bg", |b| b.iter(|| to_bg(black_box(20))));
    c.bench_function("from_hy", |b| b.iter(|| from_hy(black_box(20))));
    c.bench_function("to_hy", |b| b.iter(|| to_hy(black_box(20))));
    c.bench_function("from_ru", |b| b.iter(|| from_ru(black_box(20))));
    c.bench_function("to_ru", |b| b.iter(|| to_ru(black_box(20))));
    c.bench_function("from_ua", |b| b.iter(|| from_ua(black_box(20))));
    c.bench_function("to_ua", |b| b.iter(|| to_ua(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
