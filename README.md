# transliteratsiya

Bi-directional transliterator for Rust. Transliterates strings according to the rules specified in the language packs (source script <-> target script) and transliteration standards.

This crate uses the transliteration rules from the [Python `transliterate` package](https://github.com/barseghyanartur/transliterate) by Artur Barseghyan. 

It comes with language packs for the following languages:

- Armenian
- Bulgarian (beta)
- Greek
- Latin1
- Makedonian (alpha)
- Russian
- Serbian (alpha)
- Ukrainian (beta)

## Usage and examples

``` rust
use transliteratsiya::Bulgarian;

let t = Bulgarian::new();

let _ = t.from_latin("Lorem ipsum dolor sit amet");
let _ = t.to_latin("Лорем ипсум долор сит амет");
```

## license

LGPL-2.1-or-later
