use std::cmp::Ordering;

use crate::CharsMapping;

/// The contract for transliteration from the source language to the Latin
/// alphabet.
pub trait ToLatin {
    fn to_latin(&self, input: &str) -> String;
}

/// The contract for transliteration from the Latin alphabet to the source
/// language.
pub trait FromLatin {
    fn from_latin(&self, input: &str) -> String;
}

#[derive(Debug)]
pub struct Transliterator {
    rules: CharsMapping,
}

impl Transliterator {
    pub fn new(standard: CharsMapping) -> Self {
        let mut rules = standard;
        fn compare_len(left: &str, right: &str) -> Ordering {
            left.len().cmp(&right.len())
        }
        // sort by Latin string
        rules.sort_by(|a, b| compare_len(b.1, a.1));

        Self { rules }
    }

    pub fn translit(&self, input: &str, reverse: bool) -> String {
        let mut input = input.to_owned();

        for elem in self.rules.iter() {
            let (source_char, translit_char) = if reverse {
                (elem.1, elem.0)
            } else {
                (elem.0, elem.1)
            };

            input = input.replace(source_char, translit_char);
        }

        input
    }
}
