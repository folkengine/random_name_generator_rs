use regex::internal::Input;
bitflags! {
    pub struct Joint: u32 {
        const NONE         = 0b00000000;
        const SOME         = 0b00000001;
        const CONSONANT    = 0b00000010;
        const NO_CONSONANT = 0b00000100;
        const NO_VOWEL     = 0b00001000;
    }
}

/// 00000001 won't work with 0b00001001
///
///

/// 00000001 vowel with anything
///     00000001
///     00000011
///     00000101
///     00000111


impl Joint {
    pub fn joins(&self, to: &Joint) -> bool {
        if to.is_empty() {
            false
        } else {
            true
        }
    }

    fn joins_to(&self, to: Joint) -> bool {
        if self.contains(Joint::CONSONANT) {
            match to {
                Joint::NONE => false,
                Joint::NO_CONSONANT => false,
                Joint::NO_VOWEL => true,
                _ => true,
            }
        } else {
            match to {
                Joint::NONE => false,
                Joint::NO_CONSONANT => true,
                Joint::NO_VOWEL => false,
                _ => true,
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod join_tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input,
        case(Joint::NO_CONSONANT),
        case(Joint::SOME),
    )]
    fn joins_to(input: Joint) {
        let joint = Joint::SOME;

        assert!(joint.joins_to(input));
    }

    #[test]
    fn joint__none() {
        let joint = Joint::SOME;

        assert!(!joint.joins(&Joint::NONE));
    }

    #[test]
    fn joint__some() {
        let joint = Joint::SOME;

        let some = Joint::SOME;
        let no_consonant = Joint::SOME | Joint::NO_CONSONANT;

        assert!(joint.joins(&some));
        assert!(joint.joins(&no_consonant));
        // assert!(joint.joins(Joint::SOME | Joint::CONSONANT));
        // assert!(joint.joins(Joint::SOME | Joint::NO_CONSONANT));
        // assert!(!joint.contains(Joint::NO_VOWEL));
    }

    #[test]
    fn contains__none() {
        let joint = Joint::NONE;

        assert!(!joint.contains(Joint::SOME));
        assert!(!joint.contains(Joint::CONSONANT));
        assert!(!joint.contains(Joint::NO_CONSONANT));
        assert!(!joint.contains(Joint::NO_VOWEL));
    }
    //
    // #[test]
    // fn joint__some__no_none() {
    //     let joint = Joint::SOME;
    //
    //     assert!(!joint.contains(Joint::NONE));
    // }

    #[test]
    fn foo() {
        let join = Joint::SOME | Joint::CONSONANT;

        println!("{:?}", join);

        assert!(join.contains(Joint::NONE));
        assert_eq!(join, Joint::SOME | Joint::CONSONANT);
    }

    #[test]
    fn no_consonant() {
        let join = Joint::NONE;

        assert!(!join.contains(Joint::CONSONANT));
    }

    #[test]
    fn contains_consonant() {
        let join = Joint::CONSONANT;

        assert!(join.contains(Joint::CONSONANT));
    }
}