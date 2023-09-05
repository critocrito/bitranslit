pub mod transliterator;

pub use crate::transliterator::{Bulgarian, FromLatin, ToLatin};

pub type CharsMapping = Vec<(&'static str, &'static str)>;

#[cfg(test)]
mod tests {
    use super::*;

    const LATIN: &'static str = "Lorem ipsum dolor sit amet";
    const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";

    #[test]
    fn transliterate_bulgarian_to_latin() {
        let t = Bulgarian::new();

        let output = t.to_latin(&BULGARIAN);

        assert_eq!(output, LATIN.to_string());
    }

    // #[test]
    // fn transliterate_latin_to_bulgarian() {
    //     let t = Bulgarian::new();

    //     let output = t.from_latin(&LATIN);

    //     assert_eq!(output, BULGARIAN.to_string());
    // }
}
