use derive_builder::Builder;

type CharsMapping = Vec<(&'static str, &'static str)>;

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

#[derive(Debug, Builder, Default)]
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
    fn to_latin(&self, input: &str) -> String {
        self.translit(input, true)
    }
}

impl FromLatin for Transliterator {
    fn from_latin(&self, input: &str) -> String {
        self.translit(input, false)
    }
}
