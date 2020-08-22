use regex::internal::Input;
use std::fmt;

bitflags! {
    pub struct J: u32 {
        const NONE      = 0b00000000;
        const SOME      = 0b00000001;
        const CONSONANT = 0b00000010;
    }
}

impl J {
    fn joins(&self, to: &J) -> bool {
        println!("{}.joins({})", self, to);
        if to.is_empty() {
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
        case(J::CONSONANT),
    )]
    fn joins__some(input: J) {
        let j = J::SOME;

        assert!(j.joins(&input));
    }

    #[rstest(input,
        case(J::CONSONANT),
    )]
    fn joins__some_ne(input: J) {
        let j = J::SOME;

        assert!(!j.joins(&input));
    }

    #[test]
    fn joint__none() {
        let j = J::NONE;

        assert!(j.joins(&J::SOME));
    }
}