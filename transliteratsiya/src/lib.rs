pub mod languages;
pub mod transliterator;

pub use crate::languages::{
    armenian::Armenian, bulgarian::Bulgarian, russian::Russian, ukranian::Ukranian,
};

use crate::transliterator::{FromLatin, ToLatin, Transliterator};

pub enum SupportedLanguage {
    Armenian,
    Bulgarian,
    Russian,
    Ukranian,
}

pub fn transliterate(input: &str, language: SupportedLanguage, reverse: bool) -> String {
    let language_pack: Transliterator = match language {
        SupportedLanguage::Armenian => Armenian::new(),
        SupportedLanguage::Bulgarian => Bulgarian::new(),
        SupportedLanguage::Russian => Russian::new(),
        SupportedLanguage::Ukranian => Ukranian::new(),
    };

    match reverse {
        false => language_pack.translit(input),
        true => language_pack.translit_reverse(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transliterator::{FromLatin, ToLatin};

    const LATIN: &'static str = "Lorem ipsum dolor sit amet";
    const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";
    const ARMENIAN: &'static str = "Լօրեմ իպսում դօլօր սիտ ամետ";
    const RUSSIAN: &'static str = "Лорем ипсум долор сит амет";
    const UKRANIAN: &'static str = "Лорем іпсум долор сіт амет";

    #[test]
    fn bulgarian_to_latin() {
        let t = Bulgarian::new();

        let output = t.translit(&BULGARIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_bulgarian() {
        let t = Bulgarian::new();

        let output = t.translit_reverse(&LATIN);

        assert_eq!(output, BULGARIAN.to_string());
    }

    #[test]
    fn armenian_to_latin() {
        let t = Armenian::new();

        let output = t.translit(&ARMENIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_armenian() {
        let t = Armenian::new();

        let output = t.translit_reverse(&LATIN);

        assert_eq!(output, ARMENIAN.to_string());
    }

    #[test]
    fn russian_to_latin() {
        let t = Russian::new();

        let output = t.translit(&RUSSIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_russian() {
        let t = Russian::new();

        let output = t.translit_reverse(&LATIN);

        assert_eq!(output, RUSSIAN.to_string());
    }

    #[test]
    fn ukranian_to_latin() {
        let t = Ukranian::new();

        let output = t.translit(&UKRANIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_ukranian() {
        let t = Ukranian::new();

        let output = t.translit_reverse(&LATIN);

        assert_eq!(output, UKRANIAN.to_string());
    }
}
