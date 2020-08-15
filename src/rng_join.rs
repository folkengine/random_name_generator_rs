

bitflags! {
    pub struct Join: u32 {
        const NONE         = 0b00000000;
        const SOME         = 0b00000001;
        const CONSONANT    = 0b00000010;
        const TO_CONSONANT = 0b00000100;
        const TO_VOWEL     = 0b00001000;
    }

    pub impl Join {
        pub fn joins(&self, join: &Join)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod join_tests {
    use super::*;

    #[test]
    fn foo() {
        let join = Join::SOME | Join::CONSONANT;

        println!("{:?}", join);

        assert!(join.contains(Join::NONE));
        assert_eq!(join, Join::SOME | Join::CONSONANT);
    }

    #[test]
    fn no_consonant() {
        let join = Join::NONE;

        assert!(!join.contains(Join::CONSONANT));
    }

    #[test]
    fn contains_consonant() {
        let join = Join::CONSONANT;

        assert!(join.contains(Join::CONSONANT));
    }
}