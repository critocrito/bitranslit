//! This crate implements bi-directional transliteration for specific languages.
//! Transliteration means that, e.g. a string in Russian can be converted to the
//! latin alphabet, and also converted back to the Russian cyrillic.
//!
//! The easiest way to use this crate is to use the [`transliterate`] utility function.
//!
//! ```rust
//! use transliteratsiya::{transliterate, Language};
//!
//! # fn main() {
//! const LATIN: &'static str = "Lorem ipsum dolor sit amet";
//! const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";
//!
//! let output = transliterate(&BULGARIAN, Language::Bulgarian, false);
//! assert_eq!(output, LATIN.to_string());
//!
//! let output = transliterate(&LATIN, Language::Bulgarian, true);
//! assert_eq!(output, BULGARIAN.to_string());
//! # }
//! ```

pub mod languages;
pub mod transliterator;

use crate::{
    languages::{Armenian, Bulgarian, Greek, Latin1, Russian, Serbian, Ukranian},
    transliterator::{FromLatin, ToLatin, Transliterator},
};

pub use transliteratsiya_derive::language_pack;

/// Languages that support transliteration.
pub enum Language {
    Armenian,
    Bulgarian,
    Greek,
    Latin1,
    Russian,
    Serbian,
    Ukranian,
}

/// This utility is the prefered way to transliterate a string for any of the
/// supported languages.
///
/// ```rust
/// use transliteratsiya::{Language, transliterate};
///
/// # fn main() {
/// let _ = transliterate("Никой не е по-голям от хляба", Language::Bulgarian, false);
/// # }
/// ```
pub fn transliterate(input: &str, language: Language, reverse: bool) -> String {
    let language_pack = match language {
        Language::Armenian => Transliterator::from(Armenian::new()),
        Language::Bulgarian => Transliterator::from(Bulgarian::new()),
        Language::Greek => Transliterator::from(Greek::new()),
        Language::Latin1 => Transliterator::from(Latin1::new()),
        Language::Russian => Transliterator::from(Russian::new()),
        Language::Serbian => Transliterator::from(Serbian::new()),
        Language::Ukranian => Transliterator::from(Ukranian::new()),
    };

    match reverse {
        false => language_pack.translit(input),
        true => language_pack.translit_reverse(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LATIN: &'static str = "Lorem ipsum dolor sit amet";
    const ARMENIAN: &'static str = "Լօրեմ իպսում դօլօր սիտ ամետ";
    const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";
    const GREEK: &'static str = "Λορεμ ιψυμ δολορ σιτ αμετ";
    const RUSSIAN: &'static str = "Лорем ипсум долор сит амет";
    const SERBIAN: &'static str = "Лорем ипсум долор сит амет";
    const PANGRAM_SERBIAN: &'static str =
        "Фијуче ветар у шибљу, леди пасаже и куће иза њих и гунђа у оџацима";
    const PANGRAM_LATIN: &'static str =
        "Fijuče vetar u šiblju, ledi pasaže i kuće iza njih i gunđa u odžacima";
    const UKRANIAN: &'static str = "Лорем іпсум долор сіт амет";

    #[test]
    fn armenian_to_latin() {
        let output = transliterate(&ARMENIAN, Language::Armenian, false);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_armenian() {
        let output = transliterate(&LATIN, Language::Armenian, true);

        assert_eq!(output, ARMENIAN.to_string());
    }

    #[test]
    fn bulgarian_to_latin() {
        let output = transliterate(&BULGARIAN, Language::Bulgarian, false);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_bulgarian() {
        let output = transliterate(&LATIN, Language::Bulgarian, true);

        assert_eq!(output, BULGARIAN.to_string());
    }

    #[test]
    fn greek_to_latin() {
        let output = transliterate(&GREEK, Language::Greek, false);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_greek() {
        let output = transliterate(&LATIN, Language::Greek, true);

        assert_eq!(output, GREEK.to_string());
    }

    #[test]
    fn russian_to_latin() {
        let output = transliterate(&RUSSIAN, Language::Russian, false);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_russian() {
        let output = transliterate(&LATIN, Language::Russian, true);

        assert_eq!(output, RUSSIAN.to_string());
    }

    #[test]
    fn serbian_to_latin() {
        let output = transliterate(&SERBIAN, Language::Serbian, false);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_serbian() {
        let output = transliterate(&LATIN, Language::Serbian, true);

        assert_eq!(output, SERBIAN.to_string());
    }

    #[test]
    fn pangram_serbian_to_latin() {
        let output = transliterate(&PANGRAM_SERBIAN, Language::Serbian, false);

        assert_eq!(output, PANGRAM_LATIN.to_string());
    }

    #[test]
    fn latin_to_pangram_serbian() {
        let output = transliterate(&PANGRAM_LATIN, Language::Serbian, true);

        assert_eq!(output, PANGRAM_SERBIAN.to_string());
    }

    #[test]
    fn ukranian_to_latin() {
        let output = transliterate(&UKRANIAN, Language::Ukranian, false);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_ukranian() {
        let output = transliterate(&LATIN, Language::Ukranian, true);

        assert_eq!(output, UKRANIAN.to_string());
    }
}
