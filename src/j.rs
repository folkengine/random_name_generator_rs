use regex::internal::Input;
use std::fmt;

bitflags! {
    pub struct J: u32 {
        const NONE         = 0b00000000;
        const SOME         = 0b00000001;
        const VOWEL        = 0b00000010;
        const NO_CONSONANT = 0b00000100;
        const NO_VOWEL     = 0b00001000;
    }
}

impl J {
    fn joins(&self, to: &J) -> bool {
        println!("{}.joins({})", self, to);

        let can_to = self.joins_to(to);
        println!("can to: {}", can_to);
        let can_from = to.joins_to(self);
        println!("can from: {}", can_from);
        can_to && can_from
    }

    fn joins_to(&self, to: &J) -> bool {
        if to.is_empty() {
            println!("to is empty empty");
            false
        } else if !to.contains(J::SOME) {
            println!("no some in to");
            false
        } else if self.contains(J::VOWEL) && to.contains(J::NO_VOWEL) {
            false
        } else if !self.contains(J::VOWEL) && to.contains(J::NO_CONSONANT) {
            false
        } else {
            true
        }
    }
}

impl fmt::Display for J {
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
mod j_tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input,
        case(J::SOME),
        case(J::SOME | J::VOWEL),
    )]
    fn joins__some(input: J) {
        let j = J::SOME;

        assert!(j.joins(&input));
    }

    #[rstest(input,
        case(J::SOME | J::VOWEL),
    )]
    fn joins__no_consonant(input: J) {
        let j = J::SOME | J::NO_CONSONANT;

        assert!(j.joins(&input));
    }

    #[rstest(input,
        case(J::NONE),
        case(J::SOME),
    )]
    fn joins__no_consonant_ne(input: J) {
        let j = J::SOME | J::NO_CONSONANT;

        assert!(!j.joins(&input));
    }

    #[rstest(input,
        case(J::SOME),
        case(J::SOME | J::NO_VOWEL),
    )]
    fn joins__no_vowel(input: J) {
        let j = J::SOME | J::NO_VOWEL;

        assert!(j.joins(&input));
    }

    #[rstest(input,
        case(J::SOME | J::VOWEL),
        case(J::SOME | J::NO_CONSONANT),
    )]
    fn joins__no_vowel_ne(input: J) {
        let j = J::SOME | J::NO_VOWEL;

        assert!(!j.joins(&input));
    }

    #[test]
    fn joins__s() {
        let j = J::SOME | J::NO_CONSONANT;
        let input = J::SOME | J::VOWEL;

        assert!(j.joins(&input));
    }

    #[test]
    fn joins_to__s() {
        let j = J::SOME | J::VOWEL;
        let input = J::SOME | J::NO_CONSONANT;

        assert!(j.joins_to(&input));
    }

    #[rstest(input,
        case(J::NONE),
        case(J::VOWEL), // J::SOME is required
    )]
    fn joins__some_ne(input: J) {
        let j = J::SOME;

        assert!(!j.joins(&input));
    }

    #[rstest(input,
        case(J::NONE),
        case(J::SOME),
        case(J::VOWEL),
        case(J::SOME | J::NO_CONSONANT),
        case(J::SOME | J::NO_VOWEL),
        case(J::SOME | J::VOWEL),
        case(J::SOME | J::VOWEL | J::NO_CONSONANT),
        case(J::SOME | J::VOWEL | J::NO_VOWEL),
    )]
    fn joins__none_ne(input: J) {
        let j = J::NONE;

        assert!(!j.joins(&input));
    }

    #[test]
    fn contains() {
        let j = J::SOME;
        let input = J::VOWEL;

        assert!(!j.contains(input));
    }

    #[test]
    fn no_some() {
        let j = J::SOME;
        let input = J::VOWEL;

        assert!(!j.contains(input));
    }

    #[test]
    fn joins__no_ome() {
        let j = J::VOWEL;
        let input = J::SOME | J::VOWEL;

        assert!(!j.joins(&input));
    }

    /// delete me when all are covered
    #[test]
    fn joins__holding() {
        let j = J::SOME;
        let input = J::VOWEL;

        assert!(!j.joins(&input));
    }

    #[test]
    fn joint__none() {
        let j = J::NONE;

        assert!(!j.joins(&J::SOME));
    }
}