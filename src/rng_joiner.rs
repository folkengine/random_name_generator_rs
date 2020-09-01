use log::{debug, trace};
use std::fmt;

bitflags! {
    pub struct Joiner: u8 {
        const NONE           = 0b00000000;
        const SOME           = 0b00000001;
        const VOWEL          = 0b00000010;
        const ONLY_VOWEL     = 0b00000100;
        const ONLY_CONSONANT = 0b00001000;
    }
}

// 1  = b00000001 = Joiner::Some
// 3  = b00000011 = Joiner::Some | Joiner::VOWEL
// 5  = b00000101 = Joiner::Some | Joiner::ONLY_VOWEL
// 7  = b00000111 = Joiner::Some | Joiner::VOWEL | Joiner::ONLY_VOWEL
// 9  = b00001001 = Joiner::Some | Joiner::ONLY_CONSONANT
// 11 = b00001011 = Joiner::Some | Joiner::VOWEL | Joiner::ONLY_CONSONANT

/// Joiner is a bitflag representation of the properties that will allow for a Syllable
/// to join with another.
///
/// Joiners themselves have no awareness if they represent what's joinable previously or
/// subsequently to it. That is determined by the Syllable using it.
///
impl Joiner {
    #[allow(dead_code)]
    pub fn joins(&self, to: &Joiner) -> bool {
        debug!("{}.joins({})", self, to);

        let can_to = self.joins_to(to);
        debug!("can to: {}", can_to);
        let can_from = to.joins_to(self);
        debug!("can from: {}", can_from);
        can_to && can_from
    }

    fn joins_to(&self, to: &Joiner) -> bool {
        if to.is_empty() {
            trace!("to is empty empty");
            false
        } else if !to.contains(Joiner::SOME) {
            trace!("no some in to");
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
        debug!("value_next {:b})", self);
        if self.contains(Joiner::ONLY_CONSONANT) {
            " +c".to_string()
        } else if self.contains(Joiner::ONLY_VOWEL) {
            " +v".to_string()
        } else {
            "".to_string()
        }
    }

    pub fn value_previous(&self) -> String {
        debug!("value_previous {:b}", self);
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

    #[test]
    fn value_previous__blank() {
        let j = Joiner::SOME | Joiner::VOWEL;

        assert_eq!(j.value_previous(), "".to_string());
        assert_eq!(Joiner::SOME.value_previous(), "".to_string());
    }

    #[test]
    fn value_previous__only_consonant() {
        let j1 = Joiner::SOME | Joiner::ONLY_CONSONANT;
        let j2 = Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT;

        assert_eq!(j1.value_previous(), " -c".to_string());
        assert_eq!(j2.value_previous(), " -c".to_string());
    }

    #[test]
    fn value_previous__only_vowel() {
        let j1 = Joiner::SOME | Joiner::ONLY_VOWEL;
        let j2 = Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL;

        assert_eq!(j1.value_previous(), " -v".to_string());
        assert_eq!(j2.value_previous(), " -v".to_string());
    }

    #[test]
    fn value_next__blank() {
        let j = Joiner::SOME | Joiner::VOWEL;

        assert_eq!(j.value_next(), "".to_string());
        assert_eq!(Joiner::SOME.value_next(), "".to_string());
    }

    #[test]
    fn value_next__only_consonant() {
        let j1 = Joiner::SOME | Joiner::ONLY_CONSONANT;
        let j2 = Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT;

        assert_eq!(j1.value_next(), " +c".to_string());
        assert_eq!(j2.value_next(), " +c".to_string());
    }

    #[test]
    fn value_next__only_vowel() {
        let j1 = Joiner::SOME | Joiner::ONLY_VOWEL;
        let j2 = Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL;

        assert_eq!(j1.value_next(), " +v".to_string());
        assert_eq!(j2.value_next(), " +v".to_string());
    }

    #[test]
    fn joins_1_to_9__neg() {
        let j = Joiner::SOME;
        let t = Joiner::SOME | Joiner::ONLY_VOWEL;

        assert!(!j.joins(&t));
    }

    #[rstest(j, input,
        case(Joiner::SOME, Joiner::SOME),                                                                               // 1 to 1
        case(Joiner::SOME, Joiner::SOME | Joiner::VOWEL),                                                               // 1 to 3
        case(Joiner::SOME, Joiner::SOME | Joiner::ONLY_CONSONANT),                                                      // 1 to 9
        case(Joiner::SOME, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT),                                      // 1 to 11
        case(Joiner::SOME | Joiner::VOWEL, Joiner::SOME),                                                               // 3 to 1
        case(Joiner::SOME | Joiner::VOWEL, Joiner::SOME | Joiner::VOWEL),                                               // 3 to 3
        case(Joiner::SOME | Joiner::VOWEL, Joiner::SOME | Joiner::ONLY_VOWEL),                                          // 3 to 5
        case(Joiner::SOME | Joiner::VOWEL, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL),                          // 3 to 7
        case(Joiner::SOME | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::VOWEL),                                          // 5 to 3
        case(Joiner::SOME | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT),                 // 5 to 11
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::VOWEL),                          // 7 to 3
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL),     // 7 to 7
        case(Joiner::SOME | Joiner::ONLY_CONSONANT, Joiner::SOME),                                                      // 9 to 1
        case(Joiner::SOME | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::ONLY_CONSONANT),                             // 9 to 9
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT, Joiner::SOME),                                      // 11 to 1
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::ONLY_VOWEL),                 // 11 to 5
    )]
    fn joins_matrix(j: Joiner, input: Joiner) {
        assert!(j.joins(&input));
    }

    #[rstest(j, input,
        case(Joiner::SOME, Joiner::SOME | Joiner::ONLY_VOWEL),                                                              // 1 to 5
        case(Joiner::SOME | Joiner::VOWEL, Joiner::SOME | Joiner::ONLY_CONSONANT),                                          // 3 to 9
        case(Joiner::SOME | Joiner::VOWEL, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT),                          // 3 to 11
        case(Joiner::SOME | Joiner::ONLY_VOWEL, Joiner::SOME),                                                              // 5 to 1
        case(Joiner::SOME | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::ONLY_VOWEL),                                         // 5 to 5
        case(Joiner::SOME | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL),                         // 5 to 7
        case(Joiner::SOME | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::ONLY_CONSONANT),                                     // 5 to 9
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL, Joiner::SOME),                                              // 7 to 1
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::ONLY_VOWEL),                         // 7 to 5
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::ONLY_CONSONANT),                     // 7 to 9
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT),     // 7 to 11
        case(Joiner::SOME | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::VOWEL),                                          // 9 to 3
        case(Joiner::SOME | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::ONLY_VOWEL),                                     // 9 to 5
        case(Joiner::SOME | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL),                     // 9 to 7
        case(Joiner::SOME | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT),                 // 9 to 11
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::VOWEL),                          // 11 to 3
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_VOWEL),     // 11 to 7
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::ONLY_CONSONANT),                 // 11 to 9
        case(Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT, Joiner::SOME | Joiner::VOWEL | Joiner::ONLY_CONSONANT), // 11 to 11
    )]
    fn joins_matrix_neg(j: Joiner, input: Joiner) {
        assert!(!j.joins(&input));
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