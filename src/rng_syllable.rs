use lazy_static::lazy_static;
// use log::{info};
use regex::Regex;
use core::fmt::Alignment::Left;

static CONSONANTS: [char; 30] = [
    'b', 'ɓ', 'ʙ', 'β', 'c', 'd', 'ɗ', 'ɖ', 'ð', 'f', 'g', 'h', 'j', 'k', 'l',
    'ł', 'm', 'ɱ', 'n', 'ɳ', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
static VOWELS: [char; 36] = [
    'i', 'y', 'ɨ', 'ʉ', 'ɯ', 'u', 'ɪ', 'ʏ', 'ʊ', 'ɯ', 'ʊ', 'e', 'ø', 'ɘ', 'ɵ', 'ɤ', 'o', 'ø',
    'ə', 'ɵ', 'ɤ', 'o', 'ɛ', 'œ', 'ɜ', 'ɞ', 'ʌ', 'ɔ', 'æ', 'ɐ', 'ɞ', 'a', 'ɶ', 'ä', 'ɒ', 'ɑ'];

lazy_static! {
    static ref FULL_RE: Regex = Regex::new(r"([-+]{0,1})(\w+)\s{0,1}([\+\-][vc]){0,1}\s{0,1}([\+\-][vc]){0,1}").unwrap();
    static ref PREFIX_RE: Regex = Regex::new(r"(.+)(\-[vcVC]).*").unwrap();
    static ref SUFFIX_RE: Regex = Regex::new(r"(.+)(\+[vcCV]).*").unwrap();
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Classification {
    Prefix,
    Center,
    Suffix,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Rule {
    Consonant,
    Vowel,
    Either,
}

pub struct Syllable {
    value: String,
    classification: Classification,
    next: Rule,
    previous: Rule,
}

impl Syllable {
    // pub fn new(raw: String) -> Syllable {
    //     Syllable
    // }

    fn classify(raw: String) -> (Classification, String) {
        return (Classification::Prefix, raw);
    }

    fn to_classification(s: &str) -> Classification {
        match s {
            "-" => Classification::Prefix,
            "+" => Classification::Suffix,
            _   => Classification::Center,
        }
    }



    fn to_next_rule(s: &str) -> Rule {
        if SUFFIX_RE.is_match(s) {
            Syllable::vowel_or_consonant(s, "+v")
        } else {
            Rule::Either
        }
    }

    fn to_previous_rule(s: &str) -> Rule {
        if PREFIX_RE.is_match(s) {
            Syllable::vowel_or_consonant(s, "-v")
        } else {
            Rule::Either
        }
    }

    fn vowel_or_consonant(s: &str, matcher: &str) -> Rule {
        if s.to_ascii_lowercase().contains(matcher) {
            Rule::Vowel
        } else {
            Rule::Consonant
        }
    }
}

mod syllable_tests {
    use super::*;

    #[test]
    fn to_classification_prefix() {
        let v = "-";

        assert_eq!(Classification::Prefix, Syllable::to_classification(v));
    }

    #[test]
    fn to_classification_center() {
        let v = "";

        assert_eq!(Classification::Center, Syllable::to_classification(v));
    }

    #[test]
    fn to_classification_suffix() {
        let v = "+";

        assert_eq!(Classification::Suffix, Syllable::to_classification(v));
    }

    #[test]
    fn to_classification_garbage() {
        assert_eq!(Classification::Center, Syllable::to_classification(" "));
        assert_eq!(Classification::Center, Syllable::to_classification("asd"));
    }
}

#[cfg(test)]
mod rule_tests {

    use super::*;
    use rstest::rstest;

    #[rstest(input, expected,
        case("", Rule::Either),
        case("-ahr", Rule::Either),
        case("dus", Rule::Either),
        case("+zou ", Rule::Either),
        case("ez -c +V", Rule::Vowel),
        case("-ahr +v", Rule::Vowel),
        case("-aby +c", Rule::Consonant),
        case("dra +c", Rule::Consonant),
    )]
    fn to_next_rule(input: &str, expected: Rule) {
        assert_eq!(expected, Syllable::to_next_rule(input))
    }

    #[rstest(input, expected,
    case("", Rule::Either),
    case("-ahr", Rule::Either),
    case("dus", Rule::Either),
    case("+zou ", Rule::Either),
    case("gru -v +c", Rule::Vowel),
    case("+sakku -V", Rule::Vowel),
    case("ay -c", Rule::Consonant),
    case("it -c +v", Rule::Consonant),
    )]
    fn to_previous_rule(input: &str, expected: Rule) {
        assert_eq!(expected, Syllable::to_previous_rule(input))
    }

    // #[test]
    // fn classify_prefix() {
    //     let (classification, s) = Syllable::classify("-ansr +v".to_string());
    //
    //     assert_eq!(Classification::Prefix, classification);
    //     // assert_eq!("ansr +v".to_string(), s);
    // }


}
