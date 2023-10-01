pub mod languages;
pub mod transliterator;

pub use crate::languages::Language;
pub use crate::languages::{
    armenian::Armenian, bulgarian::Bulgarian, russian::Russian, ukranian::Ukranian,
};

pub type CharsMapping = Vec<(&'static str, &'static str)>;

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

        let output = t.to_latin(&BULGARIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_bulgarian() {
        let t = Bulgarian::new();

        let output = t.from_latin(&LATIN);

        assert_eq!(output, BULGARIAN.to_string());
    }

    #[test]
    fn armenian_to_latin() {
        let t = Armenian::new();

        let output = t.to_latin(&ARMENIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_armenian() {
        let t = Armenian::new();

        let output = t.from_latin(&LATIN);

        assert_eq!(output, ARMENIAN.to_string());
    }

    #[test]
    fn russian_to_latin() {
        let t = Russian::new();

        let output = t.to_latin(&RUSSIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_russian() {
        let t = Russian::new();

        let output = t.from_latin(&LATIN);

        assert_eq!(output, RUSSIAN.to_string());
    }

    #[test]
    fn ukranian_to_latin() {
        let t = Ukranian::new();

        let output = t.to_latin(&UKRANIAN);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn latin_to_ukranian() {
        let t = Ukranian::new();

        let output = t.from_latin(&LATIN);

        assert_eq!(output, UKRANIAN.to_string());
    }
}
