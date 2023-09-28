use std::cmp::Ordering;
use transliteratsiya_derive::{FromLatin, LanguagePack, ToLatin};

use crate::CharsMapping;

static BG: [(&str, &str); 60] = include!("standards/bg.in");
static RU: [(&str, &str); 64] = include!("standards/ru.in");
static BY: [(&str, &str); 63] = include!("standards/by.in");
static UA: [(&str, &str); 65] = include!("standards/ua.in");

/// The contract for transliteration in the Latin alphabet
pub trait ToLatin {
    fn to_latin(&self, input: &str) -> String;
}

/// The contract for transliteration from Latin alphabet
pub trait FromLatin {
    fn from_latin(&self, input: &str) -> String;
}

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

#[derive(LanguagePack, ToLatin)]
#[LanguageRules(BG)]
pub struct Bulgarian {
    translit: Transliterator,
}

#[derive(LanguagePack, ToLatin, FromLatin)]
#[LanguageRules(RU)]
pub struct Russian {
    translit: Transliterator,
}

#[derive(LanguagePack, ToLatin, FromLatin)]
#[LanguageRules(UA)]
pub struct Ukranian {
    translit: Transliterator,
}

#[derive(LanguagePack, ToLatin, FromLatin)]
#[LanguageRules(BY)]
pub struct Belarusian {
    translit: Transliterator,
}
