use regex::Regex;

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
    rule: Rule,
}

impl Syllable {
    // pub fn new(raw: String) -> Syllable {
    //     Syllable
    // }

    fn classify(raw: String) -> (Classification, String) {
        return (Classification::Prefix, raw);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_prefix() {
        let (classification, s) = Syllable::classify("-ansr +v".to_string());

        assert_eq!(Classification::Prefix, classification);
        // assert_eq!("ansr +v".to_string(), s);
    }
}
