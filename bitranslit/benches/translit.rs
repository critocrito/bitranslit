use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub use bitranslit::{transliterate, Language};

const LATIN: &str = "Lorem ipsum dolor sit amet";
const ARMENIAN: &str = "Լօրեմ իպսում դօլօր սիտ ամետ";
const BULGARIAN: &str = "Лорем ипсум долор сит амет";
const GREEK: &str = "Λορεμ ιψυμ δολορ σιτ αμετ";
const RUSSIAN: &str = "Лорем ипсум долор сит амет";
const UKRANIAN: &str = "Лорем іпсум долор сіт амет";
const SERBIAN: &str = "Лорем ипсум долор сит амет";
const PANGRAM_SERBIAN: &str = "Фијуче ветар у шибљу, леди пасаже и куће иза њих и гунђа у оџацима";
const PANGRAM_LATIN: &str = "Fijuče vetar u šiblju, ledi pasaže i kuće iza njih i gunđa u odžacima";

fn from_hy(_n: u64) {
    let _ = transliterate(ARMENIAN, Language::Armenian, false);
}

fn to_hy(_n: u64) {
    let _ = transliterate(LATIN, Language::Armenian, true);
}

fn from_bg(_n: u64) {
    let _ = transliterate(BULGARIAN, Language::Bulgarian, false);
}

fn to_bg(_n: u64) {
    let _ = transliterate(LATIN, Language::Bulgarian, true);
}

fn from_el(_n: u64) {
    let _ = transliterate(GREEK, Language::Greek, false);
}

fn to_el(_n: u64) {
    let _ = transliterate(LATIN, Language::Greek, true);
}

fn from_ru(_n: u64) {
    let _ = transliterate(RUSSIAN, Language::Russian, false);
}

fn to_ru(_n: u64) {
    let _ = transliterate(LATIN, Language::Russian, true);
}

fn from_sr(_n: u64) {
    let _ = transliterate(SERBIAN, Language::Serbian, false);
}

fn to_sr(_n: u64) {
    let _ = transliterate(LATIN, Language::Serbian, true);
}

fn from_sr_pangram(_n: u64) {
    let _ = transliterate(PANGRAM_SERBIAN, Language::Serbian, false);
}

fn to_sr_pangram(_n: u64) {
    let _ = transliterate(PANGRAM_LATIN, Language::Serbian, true);
}

fn from_ua(_n: u64) {
    let _ = transliterate(UKRANIAN, Language::Ukranian, false);
}

fn to_ua(_n: u64) {
    let _ = transliterate(LATIN, Language::Ukranian, true);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_hy", |b| b.iter(|| from_hy(black_box(20))));
    c.bench_function("to_hy", |b| b.iter(|| to_hy(black_box(20))));

    c.bench_function("from_bg", |b| b.iter(|| from_bg(black_box(20))));
    c.bench_function("to_bg", |b| b.iter(|| to_bg(black_box(20))));

    c.bench_function("from_el", |b| b.iter(|| from_el(black_box(20))));
    c.bench_function("to_el", |b| b.iter(|| to_el(black_box(20))));

    c.bench_function("from_ru", |b| b.iter(|| from_ru(black_box(20))));
    c.bench_function("to_ru", |b| b.iter(|| to_ru(black_box(20))));

    c.bench_function("from_sr", |b| b.iter(|| from_sr(black_box(20))));
    c.bench_function("to_sr", |b| b.iter(|| to_sr(black_box(20))));
    c.bench_function("from_sr_pangram", |b| {
        b.iter(|| from_sr_pangram(black_box(20)))
    });
    c.bench_function("to_sr_pangram", |b| b.iter(|| to_sr_pangram(black_box(20))));

    c.bench_function("from_ua", |b| b.iter(|| from_ua(black_box(20))));
    c.bench_function("to_ua", |b| b.iter(|| to_ua(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
