use regex::internal::Input;
use std::fmt;

bitflags! {
    pub struct Joiner: u32 {
        const NONE           = 0b00000000;
        const SOME           = 0b00000001;
        const VOWEL          = 0b00000010;
        const ONLY_VOWEL     = 0b00000100;
        const ONLY_CONSONANT = 0b00001000;
    }
}

impl Joiner {
    pub fn joins(&self, to: &Joiner) -> bool {
        println!("{}.joins({})", self, to);

        let can_to = self.joins_to(to);
        println!("can to: {}", can_to);
        let can_from = to.joins_to(self);
        println!("can from: {}", can_from);
        can_to && can_from
    }

    fn joins_to(&self, to: &Joiner) -> bool {
        if to.is_empty() {
            println!("to is empty empty");
            false
        } else if !to.contains(Joiner::SOME) {
            println!("no some in to");
            false
        } else if self.contains(Joiner::VOWEL) && to.contains(Joiner::ONLY_CONSONANT) {
            false
        } else if !self.contains(Joiner::VOWEL) && to.contains(Joiner::ONLY_VOWEL) {
            false
        } else {
            true
        }
    }

    pub fn value_next(&self) -> String {
        println!("value_next {:b})", self);
        if self.contains(Joiner::ONLY_CONSONANT) {
            " +c".to_string()
        } else if self.contains(Joiner::ONLY_VOWEL) {
            " +v".to_string()
        } else {
            "".to_string()
        }
    }

    pub fn value_previous(&self) -> String {
        println!("value_previous {:b}", self);
        if self.contains(Joiner::ONLY_CONSONANT) {
            " -c".to_string()
        } else if self.contains(Joiner::ONLY_VOWEL) {
            " -v".to_string()
        } else {
            "".to_string()
        }
    }
}

impl fmt::Display for Joiner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:b}",
            self.bits,
        )
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod joiner_tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input,
        case(Joiner::SOME),
        case(Joiner::SOME | Joiner::VOWEL),
    )]
    fn joins__some(input: Joiner) {
        let j = Joiner::SOME;

        assert!(j.joins(&input));
    }

    #[rstest(input,
        case(Joiner::SOME | Joiner::VOWEL),
    )]
    fn joins__only_vowel(input: Joiner) {
        let j = Joiner::SOME | Joiner::ONLY_VOWEL;

        assert!(j.joins(&input));
    }

    #[rstest(input,
        case(Joiner::NONE),
        case(Joiner::SOME),
    )]
    fn joins__only_vowel_ne(input: Joiner) {
        let j = Joiner::SOME | Joiner::ONLY_VOWEL;

        assert!(!j.joins(&input));
    }

    #[rstest(input,
        case(Joiner::SOME),
        case(Joiner::SOME | Joiner::ONLY_CONSONANT),
    )]
    fn joins__only_consonant(input: Joiner) {
        let j = Joiner::SOME | Joiner::ONLY_CONSONANT;

        assert!(j.joins(&input));
    }

    #[rstest(input,
        case(Joiner::SOME | Joiner::VOWEL),
        case(Joiner::SOME | Joiner::ONLY_VOWEL),
    )]
    fn joins__only_consonant_ne(input: Joiner) {
        let j = Joiner::SOME | Joiner::ONLY_CONSONANT;

        assert!(!j.joins(&input));
    }

    #[test]
    fn joins__s() {
        let j = Joiner::SOME | Joiner::ONLY_VOWEL;
        let input = Joiner::SOME | Joiner::VOWEL;

        assert!(j.joins(&input));
    }

    #[test]
    fn joins_to__s() {
        let j = Joiner::SOME | Joiner::VOWEL;
        let input = Joiner::SOME | Joiner::ONLY_VOWEL;

        assert!(j.joins_to(&input));
    }

    #[rstest(input,
        case(Joiner::NONE),
        case(Joiner::VOWEL), // Joiner::SOME is required
    )]
    fn joins__some_ne(input: Joiner) {
        let j = Joiner::SOME;

        assert!(!j.joins(&input));
    }

    #[rstest(input,
        case(Joiner::NONE),
        case(Joiner::SOME),
        case(Joiner::VOWEL),
        case(Joiner::SOME | Joiner::ONLY_VOWEL),
        case(Joiner::SOME | Joiner::ONLY_CONSONANT),
        case(Joiner::SOME | Joiner::VOWEL),
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL),
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT),
    )]
    fn joins__none_ne(input: Joiner) {
        let j = Joiner::NONE;

        assert!(!j.joins(&input));
    }

    #[test]
    fn contains() {
        let j = Joiner::SOME;
        let input = Joiner::VOWEL;

        assert!(!j.contains(input));
    }

    #[test]
    fn no_some() {
        let j = Joiner::SOME;
        let input = Joiner::VOWEL;

        assert!(!j.contains(input));
    }

    #[test]
    fn joins__no_ome() {
        let j = Joiner::VOWEL;
        let input = Joiner::SOME | Joiner::VOWEL;

        assert!(!j.joins(&input));
    }

    /// delete me when all are covered
    #[test]
    fn joins__holding() {
        let j = Joiner::SOME;
        let input = Joiner::VOWEL;

        assert!(!j.joins(&input));
    }

    #[test]
    fn joint__none() {
        let j = Joiner::NONE;

        assert!(!j.joins(&Joiner::SOME));
    }
}