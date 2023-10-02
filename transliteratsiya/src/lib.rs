pub mod languages;
pub mod transliterator;

use crate::{
    languages::{Armenian, Bulgarian, Russian, Ukranian},
    transliterator::{FromLatin, ToLatin, Transliterator},
};

/// Languages that support transliteration.
pub enum Language {
    Armenian,
    Bulgarian,
    Russian,
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
        Language::Russian => Transliterator::from(Russian::new()),
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
    const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";
    const ARMENIAN: &'static str = "Լօրեմ իպսում դօլօր սիտ ամետ";
    const RUSSIAN: &'static str = "Лорем ипсум долор сит амет";
    const UKRANIAN: &'static str = "Лорем іпсум долор сіт амет";

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
