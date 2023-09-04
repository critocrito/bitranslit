pub mod standards;

use crate::standards::bg;

pub type CharsMapping = Vec<(&'static str, &'static str)>;

pub fn translit(input: &str, invert: bool) -> String {
    let mut input = input.to_owned();
    let rules = bg::streamlined();

    for elem in rules.iter() {
        let (source_char, translit_char) = (elem.0, elem.1);

        input = {
            if invert {
                input.replace(translit_char, source_char)
            } else {
                input.replace(source_char, translit_char)
            }
        }
    }

    input
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    const LATIN: &'static str = "Lorem ipsum dolor sit amet";
    const BULGARIAN: &'static str = "Лорем ипсум долор сит амет";

    #[test]
    fn transliterate_bulgarian_to_latin() {
        let output = translit(&BULGARIAN, false);

        assert_eq!(output, LATIN.to_string());
    }

    #[test]
    fn transliterate_latin_to_bulgarian() {
        let output = translit(&LATIN, true);

        assert_eq!(output, BULGARIAN.to_string());
    }
}
