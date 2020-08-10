use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

static CONSONANTS: [char; 30] = [
    'b', 'ɓ', 'ʙ', 'β', 'c', 'd', 'ɗ', 'ɖ', 'ð', 'f', 'g', 'h', 'j', 'k', 'l',
    'ł', 'm', 'ɱ', 'n', 'ɳ', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
static VOWELS: [char; 36] = [
    'i', 'y', 'ɨ', 'ʉ', 'ɯ', 'u', 'ɪ', 'ʏ', 'ʊ', 'ɯ', 'ʊ', 'e', 'ø', 'ɘ', 'ɵ', 'ɤ', 'o', 'ø',
    'ə', 'ɵ', 'ɤ', 'o', 'ɛ', 'œ', 'ɜ', 'ɞ', 'ʌ', 'ɔ', 'æ', 'ɐ', 'ɞ', 'a', 'ɶ', 'ä', 'ɒ', 'ɑ'];

// https://regex101.com/r/kvDj4I/2/
lazy_static! {
    static ref FULL_RE: Regex = Regex::new(r"^([-+]{0,1})([A-Za-z]+)\s*([\+\-][vcVC]){0,1}\s{0,1}([\+\-][vcVC]){0,1}$").unwrap();
    static ref PREFIX_RE: Regex = Regex::new(r"(.+)(\-[vcVC]).*").unwrap();
    static ref SUFFIX_RE: Regex = Regex::new(r"(.+)(\+[vcVC]).*").unwrap();
}

#[derive(Clone, Debug, PartialEq)]
pub struct Syllable {
    pub value: String,
    pub classification: Classification,
    pub next: Rule,
    pub previous: Rule,
}

impl Syllable {
    pub fn new(raw: &str) -> Result<Syllable, BadSyllable> {
        if FULL_RE.is_match(raw) {
            let (classification, value) = Syllable::classify(raw);
            let syllable = Syllable {
                value,
                classification,
                next: Syllable::determine_next_rule(raw),
                previous: Syllable::determine_previous_rule(raw),
            };
            Ok(syllable)
        } else {
            Err(BadSyllable)
        }
    }

    pub fn ends_with_vowel(&self) -> bool {
        VOWELS.contains(&self.value.chars().last().unwrap())
    }

    pub fn starts_with_vowel(&self) -> bool {
        VOWELS.contains(&self.value.chars().next().unwrap())
    }

    fn determine_classification(s: &str) -> Classification {
        match s {
            "-" => Classification::Prefix,
            "+" => Classification::Suffix,
            _   => Classification::Center,
        }
    }

    fn determine_next_rule(s: &str) -> Rule {
        if SUFFIX_RE.is_match(s) {
            Syllable::vowel_or_consonant_flag(s, "+v")
        } else {
            Rule::Either
        }
    }

    fn determine_previous_rule(s: &str) -> Rule {
        if PREFIX_RE.is_match(s) {
            Syllable::vowel_or_consonant_flag(s, "-v")
        } else {
            Rule::Either
        }
    }

    fn vowel_or_consonant_flag(s: &str, matcher: &str) -> Rule {
        if s.to_ascii_lowercase().contains(matcher) {
            Rule::Vowel
        } else {
            Rule::Consonant
        }
    }

    fn classify(raw: &str) -> (Classification, String) {
        let cap = FULL_RE.captures(raw).unwrap();
        (
            Syllable::determine_classification(&cap[1]),
            cap[2].to_string()
        )
    }

    pub fn next(&self, syllables: &Vec<Syllable>) -> Syllable {
        return Syllable::new("boop").unwrap()
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}",
               self.classification.value(), self.value,
               self.previous.value_previous(), self.next.value_next())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BadSyllable;

impl fmt::Display for BadSyllable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Syllable")
    }
}

// region Enums

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

#[derive(Clone, Debug, PartialEq)]
pub enum Rule {
    Consonant,
    Vowel,
    Either,
}

impl Rule {
    fn value_next(&self) -> String {
        match *self {
            Rule::Consonant => " +c".to_string(),
            Rule::Vowel => " +v".to_string(),
            _ => "".to_string(),
        }
    }

    fn value_previous(&self) -> String {
        match *self {
            Rule::Consonant => " -c".to_string(),
            Rule::Vowel => " -v".to_string(),
            _ => "".to_string(),
        }
    }
}

// endregion

#[cfg(test)]
mod syllable_tests {
    use super::*;

    #[test]
    fn new__center() {
        let expected = Syllable {
            value: "idr".to_string(),
            classification: Classification::Center,
            next: Rule::Vowel,
            previous: Rule::Consonant,
        };

        let actual = Syllable::new("idr -c +v");

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn new__prefix__any() {
        let expected = Syllable {
            value: "asd".to_string(),
            classification: Classification::Prefix,
            next: Rule::Either,
            previous: Rule::Either,
        };

        let actual = Syllable::new("-asd");

        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn new__suffix__any() {
        let expected = Syllable {
            value: "adly".to_string(),
            classification: Classification::Suffix,
            next: Rule::Either,
            previous: Rule::Vowel,
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

        assert_eq!(Classification::Prefix, Syllable::determine_classification(v));
    }

    #[test]
    fn determine_classification_center() {
        let v = "";

        assert_eq!(Classification::Center, Syllable::determine_classification(v));
    }

    #[test]
    fn determine_classification_suffix() {
        let v = "+";

        assert_eq!(Classification::Suffix, Syllable::determine_classification(v));
    }

    #[test]
    fn determine_classification_garbage() {
        assert_eq!(Classification::Center, Syllable::determine_classification(" "));
        assert_eq!(Classification::Center, Syllable::determine_classification("asd"));
    }
}

#[cfg(test)]
mod rs_tests {

    use super::*;
    use rstest::rstest;
    
    fn micro() -> Vec<Syllable> {
        vec![
            Syllable::new("-a").unwrap(),
            Syllable::new("b").unwrap(),
            Syllable::new("+c").unwrap(),
        ]
    }

    #[rstest(input,
        case("!"),
        case("+-"),
        case("+123asfd3ew"),
    )]
    fn new__invalid__error(input: &str) {
        assert_eq!(Syllable::new(input).unwrap_err(), BadSyllable);
    }

    #[rstest(input,
        case("!"),
        case("+-"),
        case("+123asfd3ew"),
    )]
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

    #[rstest(input, expected,
        case(Rule::Consonant, " +c".to_string()),
        case(Rule::Vowel, " +v".to_string()),
        case(Rule::Either, "".to_string()),
    )]
    fn rule_value_next(input: Rule, expected: String) {
        assert_eq!(input.value_next(), expected);
    }

    #[rstest(input, expected,
        case(Rule::Consonant, " -c".to_string()),
        case(Rule::Vowel, " -v".to_string()),
        case(Rule::Either, "".to_string()),
    )]
    fn rule_value_previous(input: Rule, expected: String) {
        assert_eq!(input.value_previous(), expected);
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
    fn determine_next_rule(input: &str, expected: Rule) {
        assert_eq!(expected, Syllable::determine_next_rule(input));
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
    fn determine_previous_rule(input: &str, expected: Rule) {
        assert_eq!(expected, Syllable::determine_previous_rule(input))
    }
}
