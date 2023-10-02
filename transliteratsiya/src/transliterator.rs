use derive_builder::Builder;
use std::fmt;

/// A type alias for characters of a language and their counterpart in ASCII.
pub type CharsMapping = Vec<(&'static str, &'static str)>;

/// The contract for transliteration from the source language to the Latin
/// alphabet.
pub trait ToLatin {
    fn translit(&self, input: &str) -> String;
}

/// The contract for transliteration from the Latin alphabet to the source
/// language.
pub trait FromLatin {
    fn translit_reverse(&self, input: &str) -> String;
}

/// A [`Transliterator`] is at the core of transliteration. The easiest way is
/// to construct it from a language pack. Using the
/// [`transliterate`](crate::transliterate) utility function directly
/// simplifies the usage of it.
///
/// ```rust
/// use transliteratsiya::{
///     languages::Bulgarian,
///     transliterator::{Transliterator, ToLatin}
/// };
///
/// # fn main() {
/// let t = Transliterator::from(Bulgarian::new());
///
/// let output = t.translit("Никой не е по-голям от хляба");
///
/// assert_eq!(output, "Nikoy ne e po-golyam ot hlyaba".to_string())
/// # }
/// ```
///
/// Manually constructing a [`Transliterator`] requires at least a label for the
/// language and character mapping of type [`CharsMapping`].
///
/// ```rust
/// use transliteratsiya::{
///     languages::Bulgarian,
///     transliterator::{TransliteratorBuilder, ToLatin, CharsMapping}
/// };
///
/// # fn main() {
/// let language = "MyLanguage".to_string();
/// let mapping: CharsMapping = [("a", "c"), ("b", "d")].iter().cloned().collect();
///
/// let t = TransliteratorBuilder::default()
///     .language(language)
///     .mapping(mapping)
///     .pre_processor_mapping(None)
///     .reverse_specific_mapping(None)
///     .reverse_specific_pre_processor_mapping(None)
///     .build()
///     .unwrap();
///
/// let output = t.translit("ab");
///
/// assert_eq!(output, "cd".to_string())
/// # }
/// ```
///
/// Transliteration happens in 2 steps for regular transliteration into latin
/// and 4 steps for transliterations from latin alphabet into the original
/// language. Each step can associate a dedicated mapping file. Only the
/// `mapping` step requires a mapping, mappings for the other steps is optional.
///
/// ```markdown
/// if reverse:
///   reverse_specific -> reverse_specific_pre_processor -> pre_processor (swapped) -> mapping (swapped)
///
/// if not reverse:
///   pre_processor -> mapping
///
/// ```
#[derive(Debug, Builder, Clone)]
pub struct Transliterator {
    #[builder(setter(into))]
    pub language: String,
    mapping: CharsMapping,
    pre_processor_mapping: Option<CharsMapping>,
    reverse_specific_mapping: Option<CharsMapping>,
    reverse_specific_pre_processor_mapping: Option<CharsMapping>,
}

impl Transliterator {
    fn translit(&self, input: &str, reverse: bool) -> String {
        let mut input = input.to_owned();

        if reverse {
            if let Some(mapping) = &self.reverse_specific_mapping {
                for elem in mapping.iter() {
                    let (source_char, translit_char) = (elem.0, elem.1);

                    input = input.replace(source_char, translit_char);
                }
            }

            if let Some(mapping) = &self.reverse_specific_pre_processor_mapping {
                for elem in mapping.iter() {
                    let (source_char, translit_char) = (elem.0, elem.1);

                    input = input.replace(source_char, translit_char);
                }
            }
        }

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

impl ToLatin for Transliterator {
    fn translit(&self, input: &str) -> String {
        self.translit(input, false)
    }
}

impl FromLatin for Transliterator {
    fn translit_reverse(&self, input: &str) -> String {
        self.translit(input, true)
    }
}

impl fmt::Display for Transliterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.language)
    }
}
