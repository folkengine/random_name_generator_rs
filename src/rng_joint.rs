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

impl Joint {
    pub fn joins(&self, to: &Joint) -> bool {
        if to.is_empty() {
            false
        } else {
            if self.contains(Joint::CONSONANT) {
                if to.contains(Joint::NO_CONSONANT) {
                    self.contains(Joint::NO_CONSONANT)
                } else {
                    true
                }
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod join_tests {
    use super::*;

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
        let no_vowel = Joint::SOME | Joint::NO_VOWEL;

        assert!(joint.joins(&some));
        assert!(joint.joins(&no_consonant));
        assert!(!joint.joins(&no_vowel));
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