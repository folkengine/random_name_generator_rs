use rand::prelude::*;
use rand::distributions::WeightedIndex;
use std::string::ToString;
use std::fmt;

use crate::rng_syllable::Syllable;

#[derive(Debug, PartialEq)]
pub struct Dialect {
    name: String,
    syllables: Vec<Syllable>,
    bad_syllables: Vec<String>,
}

impl Dialect {
    pub fn is_valid(&self) -> bool {
        return true;
    }
}

#[derive(Debug, PartialEq)]
pub enum Dialects {
    Elven,
    Fantasy,
    Goblin,
    Roman,
}

impl fmt::Display for Dialects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Dialects {
    pub fn get_path(&self) -> String {
        format!("./src/languages/{}.txt", self.to_string())
    }
}

static SYLLABLE_COUNTS: [u8; 4] = [2, 3, 4, 5];
static SYLLABLE_WEIGHTS: [u8; 4] = [4, 10, 3, 1];

fn gen_syllable_count() -> u8 {
    let dist = WeightedIndex::new(&SYLLABLE_WEIGHTS).unwrap();
    let mut rng = thread_rng();
    SYLLABLE_COUNTS[dist.sample(&mut rng)]
}

#[cfg(test)]
mod test_weight {
    use proptest::prelude::*;
    use super::*;
    use std::string::ToString;

    proptest! {
        #[test]
        fn test_gen_syllable_count(_ in 0..100i32) {
            let count = gen_syllable_count();
            assert!((count < 6) && (count > 1), count);
        }
    }

    #[test]
    fn dialect__is_valid() {
        let elven = Dialect {
            name: "".to_string(),
            syllables: vec![],
            bad_syllables: vec![],
        };
        assert!(elven.is_valid())
    }

    #[test]
    fn dialect__is_valid__not() {
        let bad = Dialect {
            name: "bad".to_string(),
            syllables: vec![],
            bad_syllables: vec!["#$@!".to_string()]
        };
        assert!(bad.is_valid())
    }
    
    #[test]
    fn dialect_to_string() {
        assert_eq!(String::from("Elven"), Dialects::Elven.to_string());
    }

    #[test]
    fn dialect_get_path() {
        assert_eq!("./src/languages/Fantasy.txt".to_string(), Dialects::Fantasy.get_path());
    }

    //
    // #[test]
    // fn file_path() {
    //     assert_eq!("./src/languages/Demonic.txt".to_string(), lang_path(DEMONIC));
    // }
}