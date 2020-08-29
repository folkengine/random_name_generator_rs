use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use regex::Regex;
use std::fmt;

use crate::rng_joiner::{Joiner};

static _CONSONANTS: [char; 57] = [
    'b', 'ɓ', 'ʙ', 'β', 'c', 'd', 'ɗ', 'ɖ', 'ð', 'f', 'g', 'h', 'j', 'k', 'l', 'ł', 'm', 'ɱ', 'n',
    'ɳ', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
    'б', 'в', 'г', 'д', 'ж', 'з', 'к', 'л', 'м', 'н', 'п', 'р', 'с', 'т', 'ф', 'х', 'ц', 'ч', 'ш',
    'щ', 'ъ', 'ы', 'ь',
    'ѕ', 'ѳ',  'ѯ', 'ѱ', // Russian https://en.wikipedia.org/wiki/Russian_alphabet
];
static VOWELS: [char; 54] = [
    'i', 'y', 'ɨ', 'ʉ', 'ɯ', 'u', 'ɪ', 'ʏ', 'ʊ', 'ɯ', 'ʊ', 'e', 'ø', 'ɘ', 'ɵ', 'ɤ', 'o', 'ø', 'ə',
    'ɵ', 'ɤ', 'o', 'ɛ', 'œ', 'ɜ', 'ɞ', 'ʌ', 'ɔ', 'æ', 'ɐ', 'ɞ', 'a', 'ɶ', 'ä', 'ɒ', 'ɑ',
    'е', 'ё', 'э', 'и', 'й', 'ю', 'ѭ', 'я', 'ѧ', 'ѫ', 'ꙛ', 'ꙙ', 'ꙝ', 'ѩ', 'і', 'ѣ', 'ѵ', 'ѡ', // Russian
];

// https://regex101.com/r/kvDj4I/2/
lazy_static! {
    static ref FULL_RE: Regex =
        Regex::new(r"^([-+]{0,1})([A-Za-z]+)\s*([\+\-][vcVC]){0,1}\s{0,1}([\+\-][vcVC]){0,1}$")
            .unwrap();
    static ref PREFIX_RE: Regex = Regex::new(r"(.+)(\-[vcVC]).*").unwrap();
    static ref SUFFIX_RE: Regex = Regex::new(r"(.+)(\+[vcVC]).*").unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub struct Syllable {
    pub value: String,
    pub classification: Classification,
    pub jprevious: Joiner,
    pub jnext: Joiner,
}

impl Syllable {
    pub fn new(raw: &str) -> Result<Syllable, BadSyllable> {
        if FULL_RE.is_match(raw) {
            let (classification, value) = Syllable::classify(raw);
            let syllable = Syllable {
                value,
                classification,
                jnext: Syllable::determine_next_joiner(raw),
                jprevious: Syllable::determine_previous_joiner(raw),
            };
            Ok(syllable)
        } else {
            Err(BadSyllable)
        }
    }

    pub fn ends_with_vowel(&self) -> bool {
        VOWELS.contains(&self.value.chars().last().unwrap())
    }

    pub fn str_ends_with_vowel(s: &str) -> bool {
        VOWELS.contains(&s.chars().last().unwrap())
    }

    pub fn starts_with_vowel(&self) -> bool {
        VOWELS.contains(&self.value.chars().next().unwrap())
    }

    pub fn str_starts_with_vowel(s: &str) -> bool {
        VOWELS.contains(&s.chars().next().unwrap())
    }

    fn determine_classification(s: &str) -> Classification {
        match s {
            "-" => Classification::Prefix,
            "+" => Classification::Suffix,
            _ => Classification::Center,
        }
    }

    fn determine_next_joiner(s: &str) -> Joiner {
        let (_, pure) = Syllable::classify(s);
        let ends = if Syllable::str_ends_with_vowel(pure.as_str()) { Joiner::VOWEL } else { Joiner::SOME };
        if SUFFIX_RE.is_match(s) {
            Joiner::SOME | ends | Syllable::vowel_or_consonant_only_joiner(s, "+v")
        } else {
            Joiner::SOME | ends
        }
    }

    fn determine_previous_joiner(s: &str) -> Joiner {
        let (_, pure) = Syllable::classify(s);
        let starts = if Syllable::str_starts_with_vowel(pure.as_str()) { Joiner::VOWEL } else { Joiner::SOME };
        if PREFIX_RE.is_match(s) {
            Joiner::SOME | starts | Syllable::vowel_or_consonant_only_joiner(s, "-v")
        } else {
            Joiner::SOME | starts
        }
    }

    fn vowel_or_consonant_only_joiner(s: &str, matcher: &str) -> Joiner {
        if s.to_ascii_lowercase().contains(matcher) {
            Joiner::ONLY_VOWEL
        } else {
            Joiner::ONLY_CONSONANT
        }
    }

    fn classify(raw: &str) -> (Classification, String) {
        let cap = FULL_RE.captures(raw).unwrap();
        (
            Syllable::determine_classification(&cap[1]),
            cap[2].to_string(),
        )
    }

    pub fn next(&self, syllables: &Vec<Syllable>) -> Syllable {
        return syllables.choose(&mut rand::thread_rng()).unwrap().clone();
    }

    fn connects(&self, syllable: &Syllable) -> bool {
        // if (self.next == Rule::Consonant) && !syllable.starts_with_vowel() {
        //     true
        // }
        false
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.classification.value(),
            self.value,
            self.jprevious.value_previous(),
            self.jnext.value_next(),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BadSyllable;

impl fmt::Display for BadSyllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Syllable")
    }
}

// region Classification

#[derive(Clone, Debug, PartialEq)]
pub enum Classification {
    Prefix,
    Center,
    Suffix,
}

impl Classification {
    fn value(&self) -> String {
        match *self {
            Classification::Prefix => "-".to_string(),
            Classification::Suffix => "+".to_string(),
            _ => "".to_string(),
        }
    }
}

// endregion

#[cfg(test)]
#[allow(non_snake_case)]
mod syllable_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn next() {
        let b = Syllable::new("b").unwrap();
        let v = vec![b.clone()];
        let a = Syllable::new("a").unwrap();

        let actual = a.next(&v);

        assert_eq!(actual, b);
    }

    #[test]
    fn new__center() {
        let expected = Syllable {
            value: "idr".to_string(),
            classification: Classification::Center,
            jnext: Joiner::SOME | Joiner::ONLY_VOWEL,
            jprevious: Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT,
        };

        let actual = Syllable::new("idr -c +v");

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn new__prefix__any() {
        let expected = Syllable {
            value: "asd".to_string(),
            classification: Classification::Prefix,
            jnext: Joiner::SOME,
            jprevious: Joiner::SOME | Joiner::VOWEL
        };

        let actual = Syllable::new("-asd");

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn new__suffix__any() {
        let expected = Syllable {
            value: "adly".to_string(),
            classification: Classification::Suffix,
            jprevious: Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL,
            jnext: Joiner::SOME | Joiner::VOWEL,
        };

        let actual = Syllable::new("+adly -v");

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn starts_with_vowel() {
        let actual = Syllable::new("+sadly -v");

        assert_eq!(false, actual.unwrap().starts_with_vowel());
    }

    #[test]
    fn starts_with_vowel_not() {
        let actual = Syllable::new("+adly -v");

        assert_eq!(true, actual.unwrap().starts_with_vowel());
    }

    #[test]
    fn ends_with_vowel() {
        let actual = Syllable::new("+sadly -v");

        assert_eq!(true, actual.unwrap().ends_with_vowel());
    }

    #[test]
    fn ends_with_vowel_not() {
        let actual = Syllable::new("-asdf -v");

        assert_eq!(false, actual.unwrap().ends_with_vowel());
    }

    #[test]
    fn determine_classification_prefix() {
        let v = "-";

        assert_eq!(
            Classification::Prefix,
            Syllable::determine_classification(v)
        );
    }

    #[test]
    fn determine_classification_center() {
        let v = "";

        assert_eq!(
            Classification::Center,
            Syllable::determine_classification(v)
        );
    }

    #[test]
    fn determine_classification_suffix() {
        let v = "+";

        assert_eq!(
            Classification::Suffix,
            Syllable::determine_classification(v)
        );
    }

    #[test]
    fn determine_classification_garbage() {
        assert_eq!(
            Classification::Center,
            Syllable::determine_classification(" ")
        );
        assert_eq!(
            Classification::Center,
            Syllable::determine_classification("asd")
        );
    }

    fn _micro() -> Vec<Syllable> {
        vec![
            Syllable::new("-a").unwrap(),
            Syllable::new("b").unwrap(),
            Syllable::new("+c").unwrap(),
        ]
    }

    #[rstest(input, case("!"), case("+-"), case("+123asfd3ew"))]
    fn new__invalid__error(input: &str) {
        assert_eq!(Syllable::new(input).unwrap_err(), BadSyllable);
    }

    #[rstest(input, case("!"), case("+-"), case("+123asfd3ew"))]
    fn full_re(input: &str) {
        assert!(!FULL_RE.is_match(input))
    }

    #[rstest(input, expected,
        case("-ang +v", "-ang +v".to_string()),
        case("-ang +V", "-ang +v".to_string()),
        case("-ang +V -C", "-ang -c +v".to_string()),
        case("+ean -c", "+ean -c".to_string()),
        case("+emar ", "+emar".to_string()),
        case("ladd  -v +v", "ladd -v +v".to_string()),
    )]
    fn to_string(input: &str, expected: String) {
        assert_eq!(Syllable::new(input).unwrap().to_string(), expected);
    }

    #[test]
    fn to_string_tmp() {
        let s = Syllable::new("-ang +v").unwrap();

        assert_eq!(s.to_string(), "-ang +v".to_string());
    }

    #[test]
    fn value_next() {
        let syl = Syllable::new("-ang +v").unwrap();
        // assert_eq!(syl.jnext.value_next(), " +v".to_string());
        assert_eq!(syl.jprevious.value_previous(), "".to_string());
    }

    #[rstest(input, expected,
        case(Classification::Prefix, "-".to_string()),
        case(Classification::Center, "".to_string()),
        case(Classification::Suffix, "+".to_string()),
    )]
    fn classification_value(input: Classification, expected: String) {
        assert_eq!(input.value(), expected);
    }

    #[rstest(input, classification, value,
        case("+sakku -V", Classification::Suffix, "sakku".to_string()),
        case("-darr +v", Classification::Prefix, "darr".to_string()),
        case("drov", Classification::Center, "drov".to_string()),
    )]
    fn classify(input: &str, classification: Classification, value: String) {
        let (actual_classification, actual_value) = Syllable::classify(input);
        assert_eq!(classification, actual_classification);
        assert_eq!(value, actual_value);
    }
}
