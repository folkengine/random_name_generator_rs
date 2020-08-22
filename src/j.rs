use regex::internal::Input;

bitflags! {
    pub struct J: u32 {
        const NONE = 0b00000000;
        const SOME = 0b00000001;
    }
}

impl J {
    fn joins(&self, to: &J) -> bool {
        if to.is_empty() {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod j_tests {
    use super::*;
    use rstest::rstest;



    #[rstest(input,
        case(J::SOME),
    )]
    fn joins__some(input: J) {
        let j = J::SOME;

        assert!(j.joins(&input));
    }

    #[test]
    fn joint__none() {
        let j = J::NONE;

        assert!(j.joins(&J::SOME));
    }
}