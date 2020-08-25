use regex::internal::Input;
use std::fmt;

bitflags! {
    pub struct Joint: u32 {
        const NONE         = 0b00000000;
        const SOME         = 0b00000001;
        const VOWEL        = 0b00000010;
        const NO_CONSONANT = 0b00000100;
        const NO_VOWEL     = 0b00001000;
    }
}

impl Joint {
    fn joins(&self, to: &Joint) -> bool {
        println!("{}.joins({})", self, to);

        let can_to = self.joins_to(to);
        println!("can to: {}", can_to);
        let can_from = to.joins_to(self);
        println!("can from: {}", can_from);
        can_to && can_from
    }

    fn joins_to(&self, to: &Joint) -> bool {
        if to.is_empty() {
            println!("to is empty empty");
            false
        } else if !to.contains(Joint::SOME) {
            println!("no some in to");
            false
        } else if self.contains(Joint::VOWEL) && to.contains(Joint::NO_VOWEL) {
            false
        } else if !self.contains(Joint::VOWEL) && to.contains(Joint::NO_CONSONANT) {
            false
        } else {
            true
        }
    }
}

impl fmt::Display for Joint {
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
mod joint_tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input,
    case(Joint::SOME),
    case(Joint::SOME | Joint::VOWEL),
    )]
    fn joins__some(input: Joint) {
        let j = Joint::SOME;

        assert!(j.joins(&input));
    }

    #[rstest(input,
    case(Joint::SOME | Joint::VOWEL),
    )]
    fn joins__no_consonant(input: Joint) {
        let j = Joint::SOME | Joint::NO_CONSONANT;

        assert!(j.joins(&input));
    }

    #[rstest(input,
    case(Joint::NONE),
    case(Joint::SOME),
    )]
    fn joins__no_consonant_ne(input: Joint) {
        let j = Joint::SOME | Joint::NO_CONSONANT;

        assert!(!j.joins(&input));
    }

    #[rstest(input,
    case(Joint::SOME),
    case(Joint::SOME | Joint::NO_VOWEL),
    )]
    fn joins__no_vowel(input: Joint) {
        let j = Joint::SOME | Joint::NO_VOWEL;

        assert!(j.joins(&input));
    }

    #[rstest(input,
    case(Joint::SOME | Joint::VOWEL),
    case(Joint::SOME | Joint::NO_CONSONANT),
    )]
    fn joins__no_vowel_ne(input: Joint) {
        let j = Joint::SOME | Joint::NO_VOWEL;

        assert!(!j.joins(&input));
    }

    #[test]
    fn joins__s() {
        let j = Joint::SOME | Joint::NO_CONSONANT;
        let input = Joint::SOME | Joint::VOWEL;

        assert!(j.joins(&input));
    }

    #[test]
    fn joins_to__s() {
        let j = Joint::SOME | Joint::VOWEL;
        let input = Joint::SOME | Joint::NO_CONSONANT;

        assert!(j.joins_to(&input));
    }

    #[rstest(input,
    case(Joint::NONE),
    case(Joint::VOWEL), // Joint::SOME is required
    )]
    fn joins__some_ne(input: Joint) {
        let j = Joint::SOME;

        assert!(!j.joins(&input));
    }

    #[rstest(input,
    case(Joint::NONE),
    case(Joint::SOME),
    case(Joint::VOWEL),
    case(Joint::SOME | Joint::NO_CONSONANT),
    case(Joint::SOME | Joint::NO_VOWEL),
    case(Joint::SOME | Joint::VOWEL),
    case(Joint::SOME | Joint::VOWEL | Joint::NO_CONSONANT),
    case(Joint::SOME | Joint::VOWEL | Joint::NO_VOWEL),
    )]
    fn joins__none_ne(input: Joint) {
        let j = Joint::NONE;

        assert!(!j.joins(&input));
    }

    #[test]
    fn contains() {
        let j = Joint::SOME;
        let input = Joint::VOWEL;

        assert!(!j.contains(input));
    }

    #[test]
    fn no_some() {
        let j = Joint::SOME;
        let input = Joint::VOWEL;

        assert!(!j.contains(input));
    }

    #[test]
    fn joins__no_ome() {
        let j = Joint::VOWEL;
        let input = Joint::SOME | Joint::VOWEL;

        assert!(!j.joins(&input));
    }

    /// delete me when all are covered
    #[test]
    fn joins__holding() {
        let j = Joint::SOME;
        let input = Joint::VOWEL;

        assert!(!j.joins(&input));
    }

    #[test]
    fn joint__none() {
        let j = Joint::NONE;

        assert!(!j.joins(&Joint::SOME));
    }
}