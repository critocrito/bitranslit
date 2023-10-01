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
    mapping: CharsMapping,
    pre_processor_mapping: Option<CharsMapping>,
}

impl Transliterator {
    pub fn new(mapping: CharsMapping, pre_processor_mapping: Option<CharsMapping>) -> Self {
        let mut mapping = mapping;
        fn compare_len(left: &str, right: &str) -> Ordering {
            left.len().cmp(&right.len())
        }
        // sort by Latin string
        mapping.sort_by(|a, b| compare_len(b.1, a.1));

        Self {
            mapping,
            pre_processor_mapping,
        }
    }

    pub fn translit(&self, input: &str, reverse: bool) -> String {
        let mut input = input.to_owned();

        if let Some(mapping) = &self.pre_processor_mapping {
            for elem in mapping.iter() {
                let (source_char, translit_char) = if reverse {
                    (elem.1, elem.0)
                } else {
                    (elem.0, elem.1)
                };

                input = input.replace(source_char, translit_char);
            }
        }

        for elem in self.mapping.iter() {
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
