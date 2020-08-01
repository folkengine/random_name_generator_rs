use lazy_static::lazy_static;
use log::{info};
use regex::Regex;

static CONSONANTS: [char; 30] = [
    'b', 'ɓ', 'ʙ', 'β', 'c', 'd', 'ɗ', 'ɖ', 'ð', 'f', 'g', 'h', 'j', 'k', 'l',
    'ł', 'm', 'ɱ', 'n', 'ɳ', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
static VOWELS: [char; 36] = [
    'i', 'y', 'ɨ', 'ʉ', 'ɯ', 'u', 'ɪ', 'ʏ', 'ʊ', 'ɯ', 'ʊ', 'e', 'ø', 'ɘ', 'ɵ', 'ɤ', 'o', 'ø',
    'ə', 'ɵ', 'ɤ', 'o', 'ɛ', 'œ', 'ɜ', 'ɞ', 'ʌ', 'ɔ', 'æ', 'ɐ', 'ɞ', 'a', 'ɶ', 'ä', 'ɒ', 'ɑ'];

lazy_static! {
    static ref RE: Regex = Regex::new(r"(-{0,1})(\w+)\s{0,1}([\+\-][vc]){0,1}\s{0,1}([\+\-][vc]){0,1}").unwrap();
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
    None,
}

pub struct Syllable {
    value: String,
    classification: Classification,
    must_follow: Rule,
    must_proceed: Rule,
}

impl Syllable {
    // pub fn new(raw: String) -> Syllable {
    //     Syllable
    // }

    fn classify(raw: String) -> (Classification, String) {
        return (Classification::Prefix, raw);
    }
}

fn to_classification(s: &str) -> Classification {
    match s {
        "-" => Classification::Prefix,
        "+" => Classification::Suffix,
        _ => Classification::Center,
    }
}

fn to_rule(s: &str) -> Rule {
    match s {
        _ => Rule::None,
    }
}

// fn scratch(raw: String) {
//
// }

// (-{0,1})(\w+)\s{0,1}([\+\-][vc]){0,1}\s{0,1}([\+\-][vc]){0,1}

mod classification_tests {
    use super::*;

    #[test]
    fn to_classification_prefix() {
        let v = "-";

        assert_eq!(Classification::Prefix, to_classification(v));
    }

    #[test]
    fn to_classification_center() {
        let v = "";

        assert_eq!(Classification::Center, to_classification(v));
    }

    #[test]
    fn to_classification_suffix() {
        let v = "+";

        assert_eq!(Classification::Suffix, to_classification(v));
    }

    #[test]
    fn to_classification_garbage() {
        assert_eq!(Classification::Center, to_classification(" "));
        assert_eq!(Classification::Center, to_classification("asd"));
    }
}

#[cfg(test)]
mod rule_tests {

    use super::*;
    // #[test]
    // fn classify_prefix() {
    //     let (classification, s) = Syllable::classify("-ansr +v".to_string());
    //
    //     assert_eq!(Classification::Prefix, classification);
    //     // assert_eq!("ansr +v".to_string(), s);
    // }


}
